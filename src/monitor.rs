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

    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}
