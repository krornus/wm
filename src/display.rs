use crate::rect::Rect;
use crate::error::Error;
use crate::wm::Adapter;
use crate::client::Client;
use crate::window::{Window, WindowTree, WindowId};
use crate::layout::LeftMaster;

use xcb::{x, randr};

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

    pub fn open(conn: &xcb::Connection, root: x::Window) -> Result<(Vec<Monitor>, Option<usize>), Error> {
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

struct Output {
    monitor: Monitor,
    window: WindowTree,
    focus: WindowId,
}

impl Output {
    fn new(monitor: Monitor) -> Self {
        let window = WindowTree::new(LeftMaster::new());
        let root = window.root();

        Output {
            monitor: monitor,
            window: window,
            focus: root,
        }
    }

    fn add_client(&mut self, adapter: &mut Adapter, client: Client) {
        self.window.insert(&self.focus, Window::Client(client));
        self.window.arrange(adapter, &self.focus, &self.monitor.rect);
    }
}


pub struct Display {
    outputs: Vec<Output>,
    primary: Option<usize>,
    focus: Option<usize>,
}

impl Display {
    pub fn new(conn: &xcb::Connection, root: x::Window) -> Result<Self, Error> {
        let (monitors, primary) = Monitor::open(&conn, root)?;

        let outputs = monitors.into_iter()
            .map(|m| Output::new(m))
            .collect();

        Ok(Display {
            outputs: outputs,
            primary: primary,
            focus: primary,
        })
    }

    pub fn add_client(&mut self, adapter: &mut Adapter, client: Client) {
        /* TODO: handle cases with no focus (no monitor) */
        let focus = self.focus.unwrap();
        let output = &mut self.outputs[focus];

        output.add_client(adapter, client);
    }

    #[inline]
    pub fn get_client(&self, window: x::Window) -> Option<&Client> {
        self.outputs.iter()
            .find_map(|output| output.window.get_client(window))
    }

    #[inline]
    pub fn get_client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.outputs.iter_mut()
            .find_map(|output| output.window.get_client_mut(window))
    }
}
