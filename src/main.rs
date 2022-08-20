#![allow(dead_code)]
use xcb::x;

mod client;
mod display;
mod error;
mod keyboard;
mod keysym;
mod layout;
mod rect;
mod slab;
mod tag;
mod tree;
mod window;
mod wm;

use crate::tag::{Tag, Tags, TagSet, TagSetId};
use crate::display::ViewId;

#[derive(Copy, Clone)]
enum Event {
    Exit,
    ViewSet(ViewId, TagSetId, Tag),
    ViewUpdate(ViewId, TagSetId, Tag),
    Spawn(&'static str),
}

fn run(wm: &mut wm::WindowManager<Event>) -> Result<(), error::Error> {
    let mut tags = Tags::new();

    wm.bind(&keyboard::Binding {
        view: None,
        mask: keyboard::Modifier::MOD4,
        keysym: keysym::Return,
        press: keyboard::Press::Press,
        value: Event::Spawn("sakura"),
    })?;

    wm.bind(&keyboard::Binding {
        view: None,
        mask: keyboard::Modifier::MOD4,
        keysym: keysym::q,
        press: keyboard::Press::Press,
        value: Event::Exit,
    })?;

    wm.flush()?;

    loop {
        match wm.next()? {
            wm::Event::MonitorConnect(id) => {
                let view = wm.display_mut().get_view_mut(id).unwrap();
                let rect = view.rect();

                println!("connect monitor: {}", rect);

                let tagset = TagSet::new(vec!["a", "s", "d", "f", "g"]);
                let tagid = tags.insert(tagset);

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4,
                    keysym: keysym::a,
                    press: keyboard::Press::Press,
                    value: Event::ViewSet(id, tagid, Tag::On(0)),
                })?;

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4,
                    keysym: keysym::s,
                    press: keyboard::Press::Press,
                    value: Event::ViewSet(id, tagid, Tag::On(1)),
                })?;


                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT,
                    keysym: keysym::a,
                    press: keyboard::Press::Press,
                    value: Event::ViewUpdate(id, tagid, Tag::Toggle(0)),
                })?;

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT,
                    keysym: keysym::s,
                    press: keyboard::Press::Press,
                    value: Event::ViewUpdate(id, tagid, Tag::Toggle(1)),
                })?;


                wm.flush()?;
            },
            wm::Event::MonitorResize(view) => {
                wm.arrange(view, tags.masks())?;
            },
            wm::Event::ClientCreate(view, _) => {
                wm.arrange(view, tags.masks())?;
            },
            wm::Event::UserEvent(Event::Spawn(args)) => {
                wm.spawn(args);
            },
            wm::Event::UserEvent(Event::ViewSet(view, tagset, tag)) => {
                println!("set: {:?}", tag);
                tags.mask_mut(tagset).map(|mask| {
                    mask.clear();
                    mask.set(tag);
                });

                wm.arrange(view, tags.masks())?;
            },
            wm::Event::UserEvent(Event::ViewUpdate(view, tagset, tag)) => {
                println!("update: {:?}", tag);
                tags.mask_mut(tagset).map(|mask| {
                    mask.set(tag);
                });

                wm.arrange(view, tags.masks())?;
            },
            wm::Event::UserEvent(Event::Exit) => {
                break Ok(());
            },
            _ => {
            },
        }
    }
}

fn main() {
    let mut wm = wm::WindowManager::<Event>::connect(None)
        .expect("failed to connect to X11 server");

    run(&mut wm).expect("window manager error");
}
