use std::collections::HashMap;

use crate::rect::Rect;
use crate::slab::{SlabIndex, Slab};
use crate::error::Error;
use crate::wm::{Adapter, Event};
use crate::client::Client;
use crate::window::{Window, WindowTree};
use crate::layout::LeftMaster;

use xcb::{x, randr};

#[derive(Debug)]
pub struct Monitor {
    root: x::Window,
    name: String,
    primary: bool,
    rect: Rect,
}

impl Monitor {
    fn new(conn: &xcb::Connection, root: x::Window, info: &randr::MonitorInfo) -> Result<Self, Error> {

        let cookie = conn.send_request(&x::GetAtomName {
            atom: info.name(),
        });

        let reply = conn.wait_for_reply(cookie)?;

        let name = String::from(reply.name().to_utf8());
        let rect = Rect::new(info.x(), info.y(), info.width(), info.height());

        Ok(Monitor {
            root: root,
            name: name,
            primary: info.primary(),
            rect: rect,
        })
    }

    /// Get a vector of all monitors with active outputs
    pub fn query_all(conn: &xcb::Connection, root: x::Window) -> Result<(Vec<Monitor>, Option<usize>), Error> {
        let cookie = conn.send_request(&randr::GetScreenResourcesCurrent {
            window: root,
        });

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
                },
                Err(_) => { }
            }
        }

        match primary {
            Some(x) => {
                monitors[x].primary = true;
            },
            None => if monitors.len() > 0 {
                primary = Some(0);
                monitors[0].primary = true;
            }
        }


        Ok((monitors, primary))
    }

    pub fn root(&self) -> x::Window {
        self.root
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn primary(&self) -> bool {
        self.primary
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

}

pub struct View {
    monitor: Monitor,
    window: WindowTree,
    focus: usize,
}

impl View {
    fn new(monitor: Monitor) -> Self {
        let window = WindowTree::new(LeftMaster::new());
        let root = window.root();

        View {
            monitor: monitor,
            window: window,
            focus: root,
        }
    }

    pub fn arrange<T>(&mut self, adapter: &mut Adapter<T>) {
        self.window.arrange(adapter, self.focus, &self.monitor.rect);
    }

    pub fn add_client<T>(&mut self, adapter: &mut Adapter<T>, client: Client) {
        self.window.insert(self.focus, Window::Client(client));
        self.window.arrange(adapter, self.focus, &self.monitor.rect);
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ViewId {
    index: SlabIndex,
}

pub struct Display {
    root: x::Window,
    views: Slab<View>,
    primary: Option<ViewId>,
    focus: Option<ViewId>,
}

impl Display {
    pub fn new<T>(adapter: &mut Adapter<T>, root: x::Window) -> Result<Self, Error> {
        let mut display = Display {
            root: root,
            views: Slab::new(),
            primary: None,
            focus: None,
        };

        display.update(adapter)?;

        Ok(display)
    }

    fn insert(&mut self, mon: Monitor) -> ViewId {
        let view = View::new(mon);
        let index = self.views.insert(view);

        let id = ViewId {
            index: index,
        };

        if self.focus.is_none() {
            self.focus = Some(id);
        }

        id
    }

    fn set_primary<T>(&mut self, adapter: &mut Adapter<T>, id: ViewId) {
        if self.primary != Some(id) {
            self.primary = Some(id);
            adapter.push(Event::MonitorPrimary(id));
        }
    }

    pub fn configure<T>(&mut self, adapter: &mut Adapter<T>, window: x::Window) -> Result<(), Error> {
        let cookie = adapter.conn.send_request(&randr::GetScreenInfo {
            window: window,
        });

        adapter.conn.wait_for_reply(cookie)?;

        Ok(())
    }

    pub fn update<T>(&mut self, adapter: &mut Adapter<T>) -> Result<(), Error> {
        /* get all connected monitors, with the index of the primary monitor */
        let (monitors, primary) = Monitor::query_all(&adapter.conn, self.root)?;

        /* now iterate through the result, looking to pre-existing monitors */
        for (i, mon) in monitors.into_iter().enumerate() {
            let mut new = true;

            for (index, view) in self.views.iter_mut() {
                /* found a pre-existing monitor */
                if mon.name == view.monitor.name {
                    if mon.rect != view.monitor.rect {
                        view.monitor.rect = mon.rect;
                        adapter.push(Event::MonitorResize(ViewId { index }));
                    }


                    /* optionally update the primary monitor to this one */
                    if primary == Some(i) {
                        self.set_primary(adapter, ViewId { index });
                    }

                    new = false;
                    break;
                }
            }

            if new {
                /* new is true, -- we did not find it as pre-existing.
                 * here we need to check for primary monitor again */
                let id = self.insert(mon);

                adapter.push(Event::MonitorConnect(id));

                if primary == Some(i) {
                    self.set_primary(adapter, id);
                }
            }
        }

        Ok(())
    }

    pub fn add_client<T>(&mut self, adapter: &mut Adapter<T>, client: Client) {
        /* TODO: handle cases with no focus (no monitor) */
        let focus = self.focus.unwrap();
        let output = self.views.get_mut(&focus.index).unwrap();

        output.add_client(adapter, client);
    }

    #[inline]
    pub fn get_client(&self, window: x::Window) -> Option<&Client> {
        self.views.iter()
            .find_map(|(_,output)| output.window.get_client(window))
    }

    #[inline]
    pub fn get_client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.views.iter_mut()
            .find_map(|(_,output)| output.window.get_client_mut(window))
    }

    pub fn get_view(&self, id: ViewId) -> Option<&View> {
        self.views.get(&id.index)
    }

    pub fn get_view_mut(&mut self, id: ViewId) -> Option<&mut View> {
        self.views.get_mut(&id.index)
    }
}
