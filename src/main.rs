#![allow(dead_code)]
use std::collections::HashMap;

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

use crate::display::ViewId;
use crate::tag::{Tag, TagSet, TagSetId, Tags};

#[derive(Copy, Clone)]
enum Event {
    Exit,
    TagViewSet(ViewId, TagSetId, Tag),
    TagViewUpdate(ViewId, TagSetId, Tag),
    TagClientSet(TagSetId, Tag),
    TagClientUpdate(TagSetId, Tag),
    Spawn(&'static str),
}

fn run(wm: &mut wm::WindowManager<Event>) -> Result<(), error::Error> {
    let mut tags = Tags::new();
    let mut tagsets: HashMap<ViewId, Vec<TagSetId>> = HashMap::new();

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
                let view = wm.display_mut().get_mut(id).unwrap();
                let rect = view.rect();

                println!("connect monitor: {}", rect);

                let tagset = TagSet::new(vec!["a", "s", "d", "f", "g"]);
                let tagid = tags.insert(tagset);

                tagsets.entry(id).or_insert(vec![]).push(tagid);

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4,
                    keysym: keysym::a,
                    press: keyboard::Press::Press,
                    value: Event::TagViewSet(id, tagid, Tag::On(0)),
                })?;

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4,
                    keysym: keysym::s,
                    press: keyboard::Press::Press,
                    value: Event::TagViewSet(id, tagid, Tag::On(1)),
                })?;

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4 | keyboard::Modifier::CONTROL,
                    keysym: keysym::a,
                    press: keyboard::Press::Press,
                    value: Event::TagViewUpdate(id, tagid, Tag::Toggle(0)),
                })?;

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4 | keyboard::Modifier::CONTROL,
                    keysym: keysym::s,
                    press: keyboard::Press::Press,
                    value: Event::TagViewUpdate(id, tagid, Tag::Toggle(1)),
                })?;

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT,
                    keysym: keysym::a,
                    press: keyboard::Press::Press,
                    value: Event::TagClientSet(tagid, Tag::On(0)),
                })?;

                wm.bind(&keyboard::Binding {
                    view: Some(id),
                    mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT,
                    keysym: keysym::s,
                    press: keyboard::Press::Press,
                    value: Event::TagClientSet(tagid, Tag::On(1)),
                })?;

                wm.flush()?;
            }
            wm::Event::MonitorResize(view) => {
                if let Some(ids) = tagsets.get(&view) {
                    let selection = tags.select(ids);
                    wm.arrange(view, &selection)?;
                }
            }
            wm::Event::ClientCreate(view, id) => {
                wm.display_mut()
                    .get_mut(view)
                    .and_then(|view| view.get_mut(id))
                    .map(|client| client.set_mask(tags.masks().clone()));

                if let Some(ids) = tagsets.get(&view) {
                    let selection = tags.select(ids);
                    wm.arrange(view, &selection)?;
                }
            }
            wm::Event::UserEvent(Event::Spawn(args)) => {
                wm.spawn(args);
            }
            wm::Event::UserEvent(Event::TagViewSet(view, tagset, tag)) => {
                println!("set: {:?}", tag);
                tags.mask_mut(tagset).map(|mask| {
                    mask.clear();
                    mask.set(tag);
                });

                if let Some(ids) = tagsets.get(&view) {
                    let selection = tags.select(ids);
                    wm.arrange(view, &selection)?;
                }
            }
            wm::Event::UserEvent(Event::TagViewUpdate(view, tagset, tag)) => {
                println!("update: {:?}", tag);
                tags.mask_mut(tagset).map(|mask| {
                    mask.set(tag);
                });

                if let Some(ids) = tagsets.get(&view) {
                    let selection = tags.select(ids);
                    wm.arrange(view, &selection)?;
                }
            }
            wm::Event::UserEvent(Event::TagClientSet(tagset, tag)) => {
                wm.display().focus
                    .and_then(|f| wm.display_mut().get_mut(f))
                    .and_then(|v| v.get_mut(v.focus))
                    .and_then(|c| c.get_mask_mut(tagset))
                    .map(|t| {
                        println!("  -> {:?}", tag);
                        t.clear();
                        t.set(tag);
                    });

                if let Some(view) = wm.display().focus {
                    if let Some(ids) = tagsets.get(&view) {
                        let selection = tags.select(ids);
                        wm.arrange(view, &selection)?;
                    }
                }

            }
            wm::Event::UserEvent(Event::TagClientUpdate(_, _)) => {
            }
            wm::Event::UserEvent(Event::Exit) => {
                break Ok(());
            }
            _ => {}
        }
    }
}

fn main() {
    let mut wm =
        wm::WindowManager::<Event>::connect(None).expect("failed to connect to X11 server");

    run(&mut wm).expect("window manager error");
}
