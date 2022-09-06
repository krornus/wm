#![allow(dead_code)]
use std::collections::HashMap;

mod client;
mod display;
mod error;
mod keyboard;
mod keysym;
mod layout;
mod painter;
mod rect;
mod tag;
mod tree;
mod window;
mod wm;
mod slab;

use crate::display::MonitorId;
use crate::tag::{Tag, TagSet, TagSetId, Tags};
use crate::rect::{Rect, Cut};
use crate::slab::{SlabMap, AsIndex};

use xcb::x;

#[derive(Copy, Clone)]
enum Event {
    Exit,
    FocusNext,
    FocusPrevious,
    MonitorSet(MonitorId, TagSetId, Tag),
    MonitorUpdate(MonitorId, TagSetId, Tag),
    ClientSet(TagSetId, Tag),
    ClientUpdate(TagSetId, Tag),
    Spawn(&'static str),
}

struct Colorscheme {
    focus: painter::Color,
    unfocus: painter::Color,
    text: painter::Color,
    occupied: painter::Color,
    unoccupied: painter::Color,
}

impl Colorscheme {
    fn new(conn: &mut wm::Connection<Event>, wm: &mut wm::WindowManager<Event>) -> Result<Colorscheme, error::Error> {
        let painter = wm.get_painter_mut();

        Ok(Colorscheme {
            focus: painter.color(conn, 55, 146, 237)?,
            unfocus: painter.color(conn, 22, 22, 22)?,
            occupied: painter.color(conn, 55, 237, 237)?,
            unoccupied: painter.color(conn, 237, 146, 237)?,
            text: painter.white(),
        })
    }
}

struct MonitorInfo {
    bar: Rect,
    window: Rect,
    tagsets: Vec<TagSetId>,
}

impl MonitorInfo {
    const BAR_HEIGHT: u16 = 15;

    fn connect(rect: &Rect) -> Self {
        let (bar, window) = rect.cut(Cut::Horizontal(MonitorInfo::BAR_HEIGHT));

        MonitorInfo {
            bar: bar,
            window: window,
            tagsets: vec![],
        }
    }

    #[inline]
    fn tagset(&mut self, id: TagSetId) {
        self.tagsets.push(id);
    }

    fn resize(&mut self, rect: &Rect) {
        let (bar, window) = rect.cut(Cut::Horizontal(MonitorInfo::BAR_HEIGHT));

        self.bar = bar;
        self.window = window;
    }
}

struct Manager {
    conn: wm::Connection<Event>,
    wm: wm::WindowManager<Event>,
    colorscheme: Colorscheme,
    tags: Tags,
    monitors: HashMap<MonitorId, MonitorInfo>,
}

impl Manager {
    fn new(name: Option<&str>) -> Result<Self, error::Error> {
        let mut conn = wm::Connection::connect(name)?;
        let mut wm = wm::WindowManager::new(&mut conn)?;
        let colorscheme = Colorscheme::new(&mut conn, &mut wm)?;

        Ok(Manager {
            conn: conn,
            wm: wm,
            colorscheme: colorscheme,
            tags: Tags::new(),
            monitors: HashMap::new(),
        })
    }

    fn bindtag(
        &mut self,
        sym: x::Keysym,
        monitor: MonitorId,
        tag: TagSetId,
        index: usize,
    ) -> Result<(), error::Error> {
        self.wm.bind(
            &mut self.conn,
            &keyboard::Binding {
                monitor: Some(monitor),
                mask: keyboard::Modifier::MOD4,
                keysym: sym,
                press: keyboard::Press::Press,
                value: Event::MonitorSet(monitor, tag, Tag::On(index)),
            })?;

        self.wm.bind(
            &mut self.conn,
            &keyboard::Binding {
                monitor: Some(monitor),
                mask: keyboard::Modifier::MOD4 | keyboard::Modifier::CONTROL,
                keysym: sym,
                press: keyboard::Press::Press,
                value: Event::MonitorUpdate(monitor, tag, Tag::Toggle(index)),
            })?;

        self.wm.bind(
            &mut self.conn,
            &keyboard::Binding {
                monitor: Some(monitor),
                mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT,
                keysym: sym,
                press: keyboard::Press::Press,
                value: Event::ClientSet(tag, Tag::On(index)),
            })?;

        self.wm.bind(
            &mut self.conn,
            &keyboard::Binding {
                monitor: Some(monitor),
                mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT | keyboard::Modifier::CONTROL,
                keysym: sym,
                press: keyboard::Press::Press,
                value: Event::ClientUpdate(tag, Tag::Toggle(index)),
            })?;

        Ok(())
    }

    fn arrange(&mut self) -> Result<(), error::Error> {
        let conn = &mut self.conn;

        if let Some(id) = self.wm.get_monitor() {
            let info = &self.monitors[&id];
            let selection = self.tags.select(&info.tagsets);

            let mon = &mut self.wm[id];
            mon.arrange(conn, &selection)?;
        }

        Ok(())
    }

    fn drawbar(&mut self, id: MonitorId) -> Result<(), error::Error> {
        let info = &self.monitors[&id];
        let bar = info.bar;

        let mut max = 0;
        for (_, tagset) in self.tags.iter() {
            if tagset.len() > max {
                max = tagset.len();
            }
        }

        let length: u16 = (bar.w as f32 * 0.80) as u16;

        let count = self.tags.len();
        let width =  length / max as u16;
        let height = bar.h / count as u16;

        let mut square = Rect::new(bar.x, bar.y, width, height);
        let mut occsq  = Rect::new(bar.x, bar.y, width / 2, height / 2);

        let mut occupied: SlabMap<tag::TagMask> = SlabMap::new();

        for (_, client) in self.wm[id].clients() {
            let mask = client.mask();

            for (id, mask) in mask.iter() {
                let mask = match occupied.get(id) {
                    Some(m) => m.clone() | mask.clone(),
                    None => mask.clone(),
                };

                occupied.insert(id, mask);
            }
        }

        let painter = self.wm.get_painter_mut();

        for (id, tagset) in self.tags.iter() {
            let mask = &occupied.get(id.as_index());

            for (i, (_, enabled)) in tagset.tags().enumerate() {
                let occ = mask.map(|m| m.get(i))
                    .unwrap_or(false);

                if occ && enabled {
                    painter.brush(
                        &mut self.conn,
                        self.colorscheme.focus,
                        self.colorscheme.focus)?;
                } else if occ && !enabled {
                    painter.brush(
                        &mut self.conn,
                        self.colorscheme.occupied,
                        self.colorscheme.occupied)?;
                } else if !occ && enabled {
                    painter.brush(
                        &mut self.conn,
                        self.colorscheme.unoccupied,
                        self.colorscheme.unoccupied)?;
                } else {
                    painter.brush(
                        &mut self.conn,
                        self.colorscheme.unfocus,
                        self.colorscheme.unfocus)?;
                }

                painter.rect(&mut self.conn, &square)?;

                square.x += square.w as i16;
                occsq.x  += square.w as i16;
            }

            square.x = bar.x;
            square.y += height as i16;

            occsq.x = bar.x;
            occsq.y += height as i16;

        }

        Ok(())
    }

    fn run(&mut self) -> Result<(), error::Error> {
        self.wm.bind(&mut self.conn, &keyboard::Binding {
            monitor: None,
            mask: keyboard::Modifier::MOD4,
            keysym: keysym::Return,
            press: keyboard::Press::Press,
            value: Event::Spawn("xterm"),
        })?;

        self.wm.bind(&mut self.conn, &keyboard::Binding {
            monitor: None,
            mask: keyboard::Modifier::MOD4,
            keysym: keysym::q,
            press: keyboard::Press::Press,
            value: Event::Exit,
        })?;


        self.wm.bind(&mut self.conn, &keyboard::Binding {
            monitor: None,
            mask: keyboard::Modifier::MOD4,
            keysym: keysym::j,
            press: keyboard::Press::Press,
            value: Event::FocusNext,
        })?;

        self.wm.bind(&mut self.conn, &keyboard::Binding {
            monitor: None,
            mask: keyboard::Modifier::MOD4,
            keysym: keysym::k,
            press: keyboard::Press::Press,
            value: Event::FocusPrevious,
        })?;

        loop {
            match self.wm.next(&mut self.conn)? {
                wm::Event::MonitorConnect(id) => {
                    let monitor = &mut self.wm[id];
                    let rect = monitor.get_rect();

                    let mut info = MonitorInfo::connect(rect);

                    let upper = TagSet::new(&["a", "s", "d", "f", "g"]);
                    let lower = TagSet::new(&["z", "x", "c", "v", "b"]);

                    let uid = self.tags.insert(upper);
                    let lid = self.tags.insert(lower);

                    info.tagset(uid);
                    info.tagset(lid);

                    monitor.set_rect(info.window);
                    self.monitors.insert(id, info);

                    self.drawbar(id)?;

                    self.bindtag(keysym::a, id, uid, 0)?;
                    self.bindtag(keysym::s, id, uid, 1)?;
                    self.bindtag(keysym::d, id, uid, 2)?;
                    self.bindtag(keysym::f, id, uid, 3)?;
                    self.bindtag(keysym::g, id, uid, 4)?;

                    self.bindtag(keysym::z, id, lid, 0)?;
                    self.bindtag(keysym::x, id, lid, 1)?;
                    self.bindtag(keysym::c, id, lid, 2)?;
                    self.bindtag(keysym::v, id, lid, 3)?;
                    self.bindtag(keysym::b, id, lid, 4)?;
                },
                wm::Event::MonitorResize(id) => {
                    let monitor = &mut self.wm[id];
                    let info = self.monitors.get_mut(&id).unwrap();
                    let rect = monitor.get_rect();

                    info.resize(rect);
                    monitor.set_rect(info.window);
                    self.drawbar(id)?;

                    self.arrange()?;
                }
                wm::Event::UserEvent(Event::Spawn(args)) => {
                    self.wm.spawn(args);
                }
                wm::Event::ClientCreate(mid, cid) => {
                    let client = &mut self.wm[mid][cid];

                    /* assign currently selected masks to new client */
                    for (id, tagset) in self.tags.iter() {
                        client.insert_mask(id, tagset.mask().clone());
                    }

                    self.arrange()?;
                    self.drawbar(mid)?;
                }
                wm::Event::ClientDestroy(mid, _) => {
                    self.arrange()?;
                    self.drawbar(mid)?;
                }
                wm::Event::UserEvent(Event::MonitorSet(id, tid, tag)) => {
                    let tagset = &mut self.tags[tid];
                    tagset.mask_mut().clear();
                    tagset.mask_mut().set(tag);

                    self.arrange()?;
                    self.drawbar(id)?;
                }
                wm::Event::UserEvent(Event::MonitorUpdate(id, tid, tag)) => {
                    let tagset = &mut self.tags[tid];
                    tagset.mask_mut().set(tag);

                    self.arrange()?;
                    self.drawbar(id)?;
                }
                wm::Event::UserEvent(Event::ClientSet(tagset, tag)) => {
                    self.wm.get_monitor()
                        .and_then(|mid| {
                            let mon = &mut self.wm[mid];

                            mon.focus.and_then(move |focus| {
                                let map = mon[focus].mask_mut();
                                map.get_mut(tagset.as_index())
                            })
                        })
                        .map(|t| {
                            t.clear();
                            t.set(tag);
                        });

                    self.arrange()?;

                    if let Some(id) = self.wm.get_monitor() {
                        self.drawbar(id)?;
                    }
                }
                wm::Event::UserEvent(Event::ClientUpdate(tagset, tag)) => {
                    self.wm.get_monitor()
                        .and_then(|mid| {
                            let mon = &mut self.wm[mid];
                            mon.focus.and_then(move |focus| {
                                let map = mon[focus].mask_mut();
                                map.get_mut(tagset.as_index())
                            })
                        })
                        .map(|mask| mask.set(tag));

                    self.arrange()?;

                    if let Some(id) = self.wm.get_monitor() {
                        self.drawbar(id)?;
                    }
                }
                wm::Event::UserEvent(Event::FocusNext) => {
                    self.wm.next_client()
                        .map(|(mid, cid)| {
                            let display = self.wm.display_mut();
                            display.set_focus(&mut self.conn, mid, cid)
                        });

                    if let Some(id) = self.wm.get_monitor() {
                        self.drawbar(id)?;
                    }
                }
                wm::Event::UserEvent(Event::FocusPrevious) => {
                    self.wm.previous_client()
                        .map(|(mid, cid)| {
                            let display = self.wm.display_mut();
                            display.set_focus(&mut self.conn, mid, cid)
                        });

                    if let Some(id) = self.wm.get_monitor() {
                        self.drawbar(id)?;
                    }
                }
                wm::Event::UserEvent(Event::Exit) => {
                    break Ok(());
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let mut mgr = Manager::new(None)
        .expect("failed to spawn window manager");

    mgr.run()
        .expect("window manager encountered an error");
}
