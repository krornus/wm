use std::collections::HashMap;

use crate::client::Client;
use crate::error::Error;
use crate::layout::LeftMaster;
use crate::rect::Rect;
use crate::slab::{self, Slab, SlabIndex};
use crate::tag::TagSelection;
use crate::window::{Window, WindowTree};
use crate::wm::{Adapter, Event};

use xcb::{randr, x};

/// Get a vector of all monitors with active outputs
fn monitors(
    conn: &xcb::Connection,
    root: x::Window,
) -> Result<(Vec<Monitor>, Option<usize>), Error> {
    let cookie = conn.send_request(&randr::GetScreenResourcesCurrent { window: root });

    let reply = conn.wait_for_reply(cookie)?;

    let mut outputs = HashMap::new();

    for output in reply.outputs() {
        let cookie = conn.send_request(&randr::GetOutputInfo {
            output: *output,
            config_timestamp: reply.timestamp(),
        });

        let reply = conn.wait_for_reply(cookie)?;
        outputs.insert(*output, reply);
    }

    let cookie = conn.send_request(&randr::GetMonitors {
        window: root,
        get_active: true,
    });

    /* TODO: i3 checks for "cloned" monitors which have the same x,y,w,h */
    let reply = conn.wait_for_reply(cookie)?;

    let mut monitors = Vec::new();
    let mut primary = None;

    /* iterate monitors while guaranteeing exactly one primary monitor.
     * this is done by setting primary to false everywhere, and then
     * resetting primary to true for the chosen primary */
    for (i, info) in reply.monitors().enumerate() {
        let mut connected = false;
        for output in info.outputs() {
            let con = outputs.get(output).map(|x| x.connection());
            connected = con == Some(randr::Connection::Connected);

            if connected {
                break;
            }
        }

        if !connected {
            continue;
        }

        let mon = Monitor::new(conn, root, info);

        match mon {
            Ok(mut mon) => {
                if mon.primary {
                    primary = Some(i);
                }

                mon.primary = false;
                monitors.push(mon);
            }
            Err(_) => {}
        }
    }

    match primary {
        Some(x) => {
            monitors[x].primary = true;
        }
        None => {
            if monitors.len() > 0 {
                primary = Some(0);
                monitors[0].primary = true;
            }
        }
    }

    Ok((monitors, primary))
}

pub struct Monitor {
    window: x::Window,
    name: String,
    primary: bool,
    pub focus: usize,
    rect: Rect,
    tree: WindowTree,
}

impl Monitor {
    fn new(
        conn: &xcb::Connection,
        root: x::Window,
        info: &randr::MonitorInfo,
    ) -> Result<Self, Error> {
        let cookie = conn.send_request(&x::GetAtomName { atom: info.name() });

        let rect = Rect::new(info.x(), info.y(), info.width(), info.height());
        let tree = WindowTree::new(LeftMaster::new());

        let reply = conn.wait_for_reply(cookie)?;
        let name = String::from(reply.name().to_utf8());

        Ok(Monitor {
            window: root,
            name: name,
            primary: info.primary(),
            focus: tree.root(),
            rect: rect,
            tree: tree,
        })
    }

    #[inline]
    pub fn arrange<'a, 'b, T>(
        &mut self,
        adapter: &mut Adapter<T>,
        mask: &TagSelection<'a, 'b>,
    ) -> Result<(), Error> {
        match self.tree.arrange(adapter, mask, &self.rect)? {
            Some(focus) => self.focus = focus,
            None => { },
        }

        Ok(())
    }

    #[inline]
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    #[inline]
    pub fn find(&self, window: x::Window) -> Option<usize> {
        self.tree.find(window)
    }

    #[inline]
    pub fn add(&mut self, client: Client) -> usize {
        self.tree.insert(self.focus, Window::Client(client))
    }

    #[inline]
    pub fn get(&self, id: usize) -> Option<&Client> {
        self.tree.get(id)
    }

    #[inline]
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Client> {
        self.tree.get_mut(id)
    }

    #[inline]
    pub fn next(&self, id: usize) -> Option<usize> {
        self.tree.next(id)
    }

    #[inline]
    pub fn previous(&self, id: usize) -> Option<usize> {
        self.tree.previous(id)
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MonitorId {
    index: SlabIndex,
}

impl From<SlabIndex> for MonitorId {
    fn from(i: SlabIndex) -> Self {
        MonitorId { index: i }
    }
}

pub struct Display {
    root: x::Window,
    monitors: Slab<Monitor>,
    primary: Option<MonitorId>,
    pub focus: Option<MonitorId>,
}

impl Display {
    pub fn new<T>(adapter: &mut Adapter<T>, root: x::Window) -> Result<Self, Error> {
        let mut display = Display {
            root: root,
            monitors: Slab::new(),
            primary: None,
            focus: None,
        };

        display.update(adapter)?;

        Ok(display)
    }

    fn insert(&mut self, mon: Monitor) -> MonitorId {
        let index = self.monitors.insert(mon);

        let id = MonitorId { index: index };

        if self.focus.is_none() {
            self.focus = Some(id);
        }

        id
    }

    fn set_primary<T>(&mut self, adapter: &mut Adapter<T>, id: MonitorId) {
        if self.primary != Some(id) {
            self.primary = Some(id);
            adapter.push(Event::MonitorPrimary(id));
        }
    }

    pub fn reconfigure<T>(
        &mut self,
        adapter: &mut Adapter<T>,
        window: x::Window,
    ) -> Result<(), Error> {
        let cookie = adapter
            .conn
            .send_request(&randr::GetScreenInfo { window: window });

        adapter.conn.wait_for_reply(cookie)?;

        Ok(())
    }

    pub fn update<T>(&mut self, adapter: &mut Adapter<T>) -> Result<(), Error> {
        /* get all connected monitors, with the index of the primary monitor */
        let (monitors, primary) = monitors(&adapter.conn, self.root)?;

        /* now iterate through the result, looking to pre-existing monitors */
        for (i, new) in monitors.into_iter().enumerate() {
            let mut added = true;

            for (index, monitor) in self.monitors.iter_mut() {
                /* found a pre-existing monitor */
                if new.name == monitor.name {
                    if new.rect != monitor.rect {
                        monitor.rect = new.rect;
                        adapter.push(Event::MonitorResize(MonitorId { index }));
                    }

                    /* optionally update the primary monitor to this one */
                    if primary == Some(i) {
                        self.set_primary(adapter, MonitorId { index });
                    }

                    added = false;
                    break;
                }
            }

            if added {
                /* new is true, -- we did not find it as pre-existing.
                 * here we need to check for primary monitor again */
                let id = self.insert(new);

                adapter.push(Event::MonitorConnect(id));

                if primary == Some(i) {
                    self.set_primary(adapter, id);
                }
            }
        }

        Ok(())
    }
}

impl Display {
    #[inline]
    pub fn add(&mut self, client: Client) -> usize {
        /* TODO: support missing output */
        let focus = self.focus.expect("no output available");
        let output = self.monitors.get_mut(&focus.index).unwrap();

        output.add(client)
    }

    #[inline]
    pub fn get(&self, id: MonitorId) -> Option<&Monitor> {
        self.monitors.get(&id.index)
    }

    #[inline]
    pub fn get_mut(&mut self, id: MonitorId) -> Option<&mut Monitor> {
        self.monitors.get_mut(&id.index)
    }

    #[inline]
    pub fn iter(&self) -> slab::Iter<'_, Monitor> {
        self.monitors.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slab::IterMut<'_, Monitor> {
        self.monitors.iter_mut()
    }
}
