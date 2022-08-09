use crate::wm::Adapter;
use crate::rect::{Rect, Cut};
use crate::client::Client;
use crate::error::Error;
use crate::container::{Container, ContainerID};

use xcb::{x, randr};

pub struct Monitor {
    pub root: x::Window,
    pub name: String,
    pub primary: bool,
    pub rect: Rect,
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

    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}

pub struct MonitorList {
    root: x::Window,
    monitors: Vec<Monitor>,
    primary: Option<usize>,
    active: Option<usize>,
}

impl MonitorList {
    fn get_monitors(conn: &xcb::Connection, root: x::Window) -> Result<(Vec<Monitor>, Option<usize>), Error> {
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

    pub fn new(conn: &xcb::Connection, root: x::Window) -> Result<Self, Error> {
        let (monitors, primary) = Self::get_monitors(conn, root)?;

        let active = if monitors.len() > 0 {
            Some(0)
        } else {
            None
        };

        /* TODO: unsure if all flags are necessary. SCREEN_CHANGE is the only one
         * we need to watch in order to call update() */
        conn.send_and_check_request(&randr::SelectInput {
            window: root,
            enable: randr::NotifyMask::SCREEN_CHANGE |
                    randr::NotifyMask::OUTPUT_CHANGE |
                    randr::NotifyMask::CRTC_CHANGE |
                    randr::NotifyMask::OUTPUT_PROPERTY
        })?;

        Ok(MonitorList {
            root: root,
            active: active,
            primary: primary,
            monitors: monitors
        })
    }

    pub fn update(&mut self, adapter: &mut Adapter) -> Result<(), Error> {
        let name = self.active.map(|i| String::from(&self.monitors[i].name));

        let (monitors, primary) = Self::get_monitors(&adapter.conn, self.root)?;

        self.monitors = monitors;
        self.primary = primary;

        let mut active = name.and_then(
            |n| self.monitors.iter().position(|m| m.name == n)
        );

        if active.is_none() && self.monitors.len() > 0 {
            active = Some(0);
        }

        self.active = active;

        Ok(())
    }

    #[inline]
    pub fn primary(&self) -> Option<&Monitor> {
        self.primary.map(|x| &self.monitors[x])
    }

    #[inline]
    fn index(&self) -> Option<usize> {
        self.active.or(self.primary)
    }

    #[inline]
    pub fn active(&self) -> Option<&Monitor> {
        self.index().map(|x| &self.monitors[x])
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Monitor> {
        self.monitors.iter()
    }
}

pub struct Display {
    root: Container,
    monitors: MonitorList,
    bars: Vec<ContainerID>,
    outputs: Vec<ContainerID>,
}

impl Display {
    pub fn new(conn: &xcb::Connection, root: x::Window) -> Result<Self, Error> {
        let monitors = MonitorList::new(&conn, root)?;
        let mut root = Container::new(Rect::new(0, 0, 0, 0,));

        let mut bars = Vec::with_capacity(monitors.monitors.len());
        let mut outputs = Vec::with_capacity(monitors.monitors.len());

        for mon in monitors.iter() {
            let con = root.scope(*mon.rect());
            let (bar, win) = con.rect().cut(Cut::Horizontal(20));

            println!("monitor: {}", mon.rect);
            println!("  bar: {}", bar);
            println!("  scope: {}", win);

            bars.push(con.scope(bar).id());
            outputs.push(con.scope(win).id());
        }

        Ok(Display {
            root: root,
            monitors: monitors,
            bars: bars,
            outputs: outputs
        })
    }

    pub fn primary(&self) -> ContainerID {
        self.monitors.primary
            .map(|i| self.outputs[i])
            .unwrap_or(self.root.id())
    }

    pub fn active(&self) -> ContainerID {
        self.monitors.index()
            .map(|i| self.outputs[i])
            .unwrap_or(self.root.id())
    }

    pub fn client(&mut self, window: x::Window, rect: Rect) -> Result<(), Error> {
        match self.root.by_window(window) {
            Some(_) => {
                Ok(())
            },
            None => {
                let id = self.active();
                self.root.add(id, window, rect)?;
                Ok(())
            }
        }
    }
}
