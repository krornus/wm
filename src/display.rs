use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::client::Client;
use crate::error::Error;
use crate::layout::{Layout, LeftMaster};
use crate::rect::Rect;
use crate::tag::TagSelection;
use crate::window::{WindowTree, ClientId, LayoutId, AsRawIndex, Window};
use crate::wm::{Connection, Event};

use xcb::{randr, x};

/// Get a vector of all monitors with active outputs
fn monitors<T>(
    conn: &Connection<T>,
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

/// A Monitor is a combination of a WindowTree and an actual monitor output.
/// It mostly mirrors the WindowTree API, but adds some metadata relating
/// to the actual monitor such as size and name.
pub struct Monitor {
    window: x::Window,
    name: String,
    primary: bool,
    pub focus: Option<ClientId>,
    rect: Rect,
    tree: WindowTree,
}

impl Monitor {
    fn new<T>(
        conn: &Connection<T>,
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
            focus: None,
            rect: rect,
            tree: tree,
        })
    }

    pub fn get_rect(&mut self) -> &Rect {
        &self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl Monitor {
    /* these functions mostly mirror the WindowTree API, with a few minor tweaks */

    #[inline]
    pub fn arrange<'a, 'b, T>(
        &mut self,
        conn: &mut Connection<T>,
        mask: &TagSelection<'a, 'b>,
    ) -> Result<(), Error> {
        self.focus = self.tree.arrange(conn, mask, &self.rect)?;
        Ok(())
    }

    fn focused_layout(&self) -> LayoutId {
        match self.focus {
            /* clients are guarenteed to have a parent */
            Some(client) => self.tree.parent(client).unwrap(),
            None => self.tree.root(),
        }
    }

    #[inline]
    pub fn client(&mut self, client: Client) -> ClientId {
        self.tree.client(self.focused_layout(), client)
    }

    #[inline]
    pub fn layout(&mut self, layout: impl Layout + 'static) -> LayoutId {
        self.tree.layout(self.focused_layout(), layout)
    }

    #[inline]
    pub fn find(&self, window: x::Window) -> Option<ClientId> {
        self.tree.find(window)
    }

    #[inline]
    pub fn remove<I: AsRawIndex>(&mut self, id: I) -> Window {
        self.tree.remove(id)
    }

    #[inline]
    pub fn parent<I: AsRawIndex>(&self, i: I) -> Option<LayoutId> {
        self.tree.parent(i)
    }

    #[inline]
    pub fn next_client<I: AsRawIndex>(&self, i: I) -> Option<ClientId> {
        self.tree.next_client(i)
    }


    #[inline]
    pub fn previous_client<I: AsRawIndex>(&self, i: I) -> Option<ClientId> {
        self.tree.previous_client(i)
    }


    #[inline]
    pub fn next_layout<I: AsRawIndex>(&self, i: I) -> Option<LayoutId> {
        self.tree.next_layout(i)
    }


    #[inline]
    pub fn previous_layout<I: AsRawIndex>(&self, i: I) -> Option<LayoutId> {
        self.tree.previous_layout(i)
    }
}


impl Index<ClientId> for Monitor {
    type Output = Client;

    #[inline]
    fn index(&self, index: ClientId) -> &Self::Output {
        &self.tree[index]
    }
}

impl IndexMut<ClientId> for Monitor {
    #[inline]
    fn index_mut(&mut self, index: ClientId) -> &mut Self::Output {
        &mut self.tree[index]
    }
}


impl Index<LayoutId> for Monitor {
    type Output = dyn Layout;

    #[inline]
    fn index(&self, index: LayoutId) -> &Self::Output {
        &self.tree[index]
    }
}

impl IndexMut<LayoutId> for Monitor {
    #[inline]
    fn index_mut(&mut self, index: LayoutId) -> &mut Self::Output {
        &mut self.tree[index]
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MonitorId {
    inner: usize,
}

pub struct Display {
    root: x::Window,
    monitors: slab::Slab<Monitor>,
    primary: Option<MonitorId>,
    focus: Option<MonitorId>,
}

impl Display {
    pub fn new<T>(conn: &mut Connection<T>, root: x::Window) -> Result<Self, Error> {
        let mut display = Display {
            root: root,
            monitors: slab::Slab::new(),
            primary: None,
            focus: None,
        };

        display.update(conn)?;

        Ok(display)
    }

    fn insert(&mut self, mon: Monitor) -> MonitorId {
        let index = self.monitors.insert(mon);

        let id = MonitorId { inner: index };

        if self.focus.is_none() {
            self.focus = Some(id);
        }

        id
    }

    fn set_primary<T>(&mut self, conn: &mut Connection<T>, id: MonitorId) {
        if self.primary != Some(id) {
            self.primary = Some(id);
            conn.push(Event::MonitorPrimary(id));
        }
    }

    pub fn reconfigure<T>(
        &mut self,
        conn: &mut Connection<T>,
        window: x::Window,
    ) -> Result<(), Error> {
        let cookie = conn
            .send_request(&randr::GetScreenInfo { window: window });

        conn.wait_for_reply(cookie)?;

        Ok(())
    }

    pub fn update<T>(&mut self, conn: &mut Connection<T>) -> Result<(), Error> {
        /* get all connected monitors, with the index of the primary monitor */
        let (monitors, primary) = monitors(&conn, self.root)?;

        /* now iterate through the result, looking to pre-existing monitors */
        for (i, new) in monitors.into_iter().enumerate() {
            let mut added = true;

            for (index, monitor) in self.monitors.iter_mut() {
                /* found a pre-existing monitor */
                if new.name == monitor.name {
                    if new.rect != monitor.rect {
                        monitor.rect = new.rect;
                        conn.push(Event::MonitorResize(MonitorId { inner: index }));
                    }

                    /* optionally update the primary monitor to this one */
                    if primary == Some(i) {
                        self.set_primary(conn, MonitorId { inner: index });
                    }

                    added = false;
                    break;
                }
            }

            if added {
                /* new is true, -- we did not find it as pre-existing.
                 * here we need to check for primary monitor again */
                let id = self.insert(new);

                conn.push(Event::MonitorConnect(id));

                if primary == Some(i) {
                    self.set_primary(conn, id);
                }
            }
        }

        Ok(())
    }
}

impl Display {
    #[inline]
    pub fn client(&mut self, client: Client) -> ClientId {
        /* TODO: support missing output */
        let focus = self.focus.expect("no output available");
        let output = self.monitors.get_mut(focus.inner).unwrap();

        output.client(client)
    }

    pub fn set_focus<T>(&mut self, conn: &mut Connection<T>, id: MonitorId, client: ClientId) -> Result<(), Error> {
        let mon = &mut self[id];

        mon[client].focus(conn)?;
        mon.focus = Some(client);

        Ok(())
    }

    #[inline]
    pub fn get_focus(&mut self) -> Option<MonitorId> {
        self.focus
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.monitors.iter()
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            iter: self.monitors.iter_mut()
        }
    }
}

impl Index<MonitorId> for Display {
    type Output = Monitor;

    #[inline]
    fn index(&self, index: MonitorId) -> &Self::Output {
        self.monitors.get(index.inner)
            .expect("invalid monitor id!")
    }
}

impl IndexMut<MonitorId> for Display {
    #[inline]
    fn index_mut(&mut self, index: MonitorId) -> &mut Self::Output {
        self.monitors.get_mut(index.inner)
            .expect("invalid monitor id!")
    }
}


pub struct Iter<'a> {
    iter: slab::Iter<'a, Monitor>
}

impl<'a> Iterator for Iter<'a> {
    type Item =  (MonitorId, &'a Monitor);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i,m)| {
            (MonitorId { inner: i }, m)
        })
    }
}

pub struct IterMut<'a> {
    iter: slab::IterMut<'a, Monitor>
}

impl<'a> Iterator for IterMut<'a> {
    type Item =  (MonitorId, &'a mut Monitor);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i,m)| {
            (MonitorId { inner: i }, m)
        })
    }
}
