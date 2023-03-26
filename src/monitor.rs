use std::collections::HashMap;

use xcb::x;
use xcb::randr::{self, Output};
use slab::Slab;

use crate::comm::Event;
use crate::rect::Rect;
use crate::error::Error;
use crate::manager::Connection;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct MonitorId {
    id: usize,
}

pub struct Monitor {
    root: x::Window,
    name: String,
    size: Rect,
}

pub struct Monitors {
    monitors: Slab<Monitor>,
}

fn outputs(conn: &Connection, root: x::Window) -> Result<HashMap<Output, randr::Connection>, Error> {
    let cookie = conn.send_request(&randr::GetScreenResourcesCurrent {
        window: root
    });

    let reply = conn.wait_for_reply(cookie)?;

    let mut outputs = HashMap::new();

    for output in reply.outputs() {
        let cookie = conn.send_request(&randr::GetOutputInfo {
            output: *output,
            config_timestamp: reply.timestamp(),
        });

        let reply = conn.wait_for_reply(cookie)?;
        outputs.insert(*output, reply.connection());
    }

    Ok(outputs)
}


impl Monitors {
    pub fn new() -> Self {
        Monitors {
            monitors: Slab::new(),
        }
    }

    fn add(&mut self, conn: &mut Connection, root: x::Window, mon: Monitor) -> MonitorId {
        let mut id;
        let name = &mon.name;

        for (k, v) in self.monitors.iter_mut() {
            if &v.root == &root && &v.name == name {

                id = MonitorId { id: k };

                if &v.size != &mon.size {
                    conn.push(Event::MonitorSize {
                        monitor: id,
                        x: mon.size.x,
                        y: mon.size.y,
                        width: mon.size.w,
                        height: mon.size.h,
                    });
                }

                *v = mon;
                return id;
            }
        }

        id = MonitorId { id: self.monitors.vacant_key() };
        conn.push(Event::MonitorConnect {
            monitor: id,
            x: mon.size.x,
            y: mon.size.y,
            width: mon.size.w,
            height: mon.size.h,
        });

        self.monitors.insert(mon);

        id
    }

    pub fn update(&mut self, conn: &mut Connection, root: x::Window) -> Result<(), Error> {
        self.monitors.shrink_to_fit();

        let cookie = conn.send_request(&randr::GetMonitors {
            window: root,
            get_active: true,
        });

        let outputs = outputs(conn, root)?;
        let reply = conn.wait_for_reply(cookie)?;

        let mut disconnected = vec![false; self.monitors.capacity()];
        for (k, _) in self.monitors.iter() {
            disconnected[k] = true;
        }

        for (i, info) in reply.monitors().enumerate() {
            let mut connected = false;

            for output in info.outputs() {
                match outputs.get(output) {
                    Some(randr::Connection::Connected) => {
                        connected = true;
                        break;
                    },
                    _ => { }
                }
            }

            if !connected {
                continue;
            }

            let cookie = conn.send_request(&x::GetAtomName { atom: info.name() });

            let size = Rect::new(info.x(), info.y(), info.width(), info.height());

            let reply = conn.wait_for_reply(cookie)?;
            let name = String::from(reply.name().to_utf8());

            let id = self.add(conn, root, Monitor {
                root,
                name,
                size,
            });

            if id.id < disconnected.len() {
                disconnected[id.id] = false;
            }

            if info.primary() {
                conn.push(Event::MonitorPrimary {
                    monitor: id,
                });
            }
        }

        for (i, dc) in disconnected.into_iter().enumerate() {
            if dc {
                self.monitors.remove(i);

                conn.push(Event::MonitorDisconnect {
                    monitor: MonitorId { id: i },
                });
            }
        }

        Ok(())
    }
}
