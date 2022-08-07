use crate::wm::Adapter;
use crate::rect::Rect;
use crate::error::Error;

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
}

pub struct MonitorManager {
    root: x::Window,
    active: Option<usize>,
    monitors: Vec<Monitor>,
}

impl MonitorManager {
    fn get_monitors(conn: &xcb::Connection, root: x::Window) -> Result<Vec<Monitor>, Error> {
        let cookie = conn.send_request(&randr::GetMonitors {
            window: root,
            get_active: true,
        });

        /* TODO: i3 checks for "cloned" monitors which have the same x,y,w,h */
        let reply = conn.wait_for_reply(cookie)?;
        let mut monitors: Vec<_> = reply.monitors()
                .filter_map(|x| Monitor::new(conn, root, x).ok())
                .collect();

        /* ensure we have a primary monitor */
        if monitors.len() == 1 {
            monitors[0].primary = true;
        } else if monitors.len() > 0 && !monitors.iter().any(|m| m.primary) {
            monitors[0].primary = true;
        }

        Ok(monitors)
    }

    pub fn new(conn: &xcb::Connection, root: x::Window) -> Result<Self, Error> {
        let monitors = Self::get_monitors(conn, root)?;

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

        Ok(MonitorManager {
            root: root,
            active: active,
            monitors: monitors
        })
    }

    pub fn update(&mut self, adapter: &mut Adapter) -> Result<(), Error> {
        let name = self.active.map(|i| String::from(&self.monitors[i].name));

        self.monitors = Self::get_monitors(&adapter.conn, self.root)?;

        let mut active = name.and_then(
            |n| self.monitors.iter().position(|m| m.name == n)
        );

        if active.is_none() && self.monitors.len() > 0 {
            active = Some(0);
        }

        self.active = active;

        Ok(())
    }

    pub fn active(&self) -> Option<&Monitor> {
        self.active.map(|x| &self.monitors[x])
    }

    pub fn active_mut(&mut self) -> Option<&mut Monitor> {
        self.active.map(move |x| &mut self.monitors[x])
    }
}
