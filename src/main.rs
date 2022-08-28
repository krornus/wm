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
mod slab;
mod tag;
mod tree;
mod window;
mod wm;

use crate::display::MonitorId;
use crate::tag::{Tag, TagSet, TagSetId, Tags};
use crate::rect::{Rect, Cut};

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
}

impl Colorscheme {
    fn new(conn: &mut wm::Connection<Event>, wm: &mut wm::WindowManager<Event>) -> Result<Colorscheme, error::Error> {
        let painter = wm.get_painter_mut();

        Ok(Colorscheme {
            focus: painter.color(conn, 80, 250, 123)?,
            unfocus: painter.black(),
            text: painter.black(),
        })
    }
}

struct MonitorInfo {
    bar: Rect,
    window: Rect,
    tagsets: Vec<TagSetId>,
}

impl MonitorInfo {
    const BAR_HEIGHT: u16 = 30;

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

            self.wm.get_mut(id)
                .map(|m| m.arrange(conn, &selection));
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
                    let monitor = self.wm.get_mut(id).unwrap();
                    let rect = monitor.get_rect();

                    let mut info = MonitorInfo::connect(rect);

                    let upper = TagSet::new(vec!["a", "s", "d", "f", "g"]);
                    let lower = TagSet::new(vec!["z", "x", "c", "v", "b"]);

                    let uid = self.tags.insert(upper);
                    let lid = self.tags.insert(lower);

                    info.tagset(uid);
                    info.tagset(lid);

                    monitor.set_rect(info.window);
                    self.monitors.insert(id, info);

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
                    let monitor = self.wm.get_mut(id).unwrap();

                    let info = self.monitors.get_mut(&id).unwrap();
                    let rect = monitor.get_rect();

                    info.resize(rect);
                    monitor.set_rect(info.window);

                    self.arrange()?;
                }
                wm::Event::UserEvent(Event::Spawn(args)) => {
                    self.wm.spawn(args);
                }
                wm::Event::ClientCreate(mid, cid) => {
                    let mask = self.tags.masks().clone();

                    self.wm.get_mut(mid)
                        .and_then(|mon| mon.get_mut(cid))
                        .map(|client| client.set_mask(mask));

                    self.arrange()?;
                }
                wm::Event::UserEvent(Event::MonitorSet(_, tagset, tag)) => {
                    self.tags.mask_mut(tagset).map(|mask| {
                        mask.clear();
                        mask.set(tag);
                    });

                    self.arrange()?;
                }
                wm::Event::UserEvent(Event::MonitorUpdate(_, tagset, tag)) => {
                    self.tags.mask_mut(tagset).map(|mask| {
                        mask.set(tag);
                    });

                    self.arrange()?;
                }
                wm::Event::UserEvent(Event::ClientSet(tagset, tag)) => {
                    self.wm.get_monitor()
                        .and_then(|f| self.wm.get_mut(f))
                        .and_then(|v| v.get_mut(v.focus))
                        .and_then(|c| c.get_mask_mut(tagset))
                        .map(|t| {
                            t.clear();
                            t.set(tag);
                        });

                    self.arrange()?;
                }
                wm::Event::UserEvent(Event::ClientUpdate(tagset, tag)) => {
                    self.wm.get_monitor()
                        .and_then(|focus| self.wm.get_mut(focus))
                        .and_then(|mon| mon.get_mut(mon.focus))
                        .and_then(|client| client.get_mask_mut(tagset))
                        .map(|mask| mask.set(tag));

                    self.arrange()?;
                }
                wm::Event::UserEvent(Event::FocusNext) => {
                    self.wm.get_monitor()
                        .and_then(|f| self.wm.get(f).map(|v| (f, v)))
                        .and_then(|(f, v)| v.next(v.focus).map(|c| (f, c)))
                        .map(|(f, c)| self.wm.display_mut().set_focus(&mut self.conn, f, c));
                }
                wm::Event::UserEvent(Event::FocusPrevious) => {
                    if let Some(mid) =  self.wm.get_monitor() {
                        self.wm.get(mid)
                            .and_then(|mon| mon.previous(mon.focus))
                            .map(|cid| self.wm.display_mut().set_focus(&mut self.conn, mid, cid));
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
