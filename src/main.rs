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

use crate::display::MonitorId;
use crate::tag::{Tag, TagSet, TagSetId, Tags};

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

fn tag(
    wm: &mut wm::WindowManager<Event>,
    sym: x::Keysym,
    view: MonitorId,
    tag: TagSetId,
    index: usize,
) -> Result<(), error::Error> {
    wm.bind(&keyboard::Binding {
        view: Some(view),
        mask: keyboard::Modifier::MOD4,
        keysym: sym,
        press: keyboard::Press::Press,
        value: Event::MonitorSet(view, tag, Tag::On(index)),
    })?;

    wm.bind(&keyboard::Binding {
        view: Some(view),
        mask: keyboard::Modifier::MOD4 | keyboard::Modifier::CONTROL,
        keysym: sym,
        press: keyboard::Press::Press,
        value: Event::MonitorUpdate(view, tag, Tag::Toggle(index)),
    })?;

    wm.bind(&keyboard::Binding {
        view: Some(view),
        mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT,
        keysym: sym,
        press: keyboard::Press::Press,
        value: Event::ClientSet(tag, Tag::On(index)),
    })?;

    wm.bind(&keyboard::Binding {
        view: Some(view),
        mask: keyboard::Modifier::MOD4 | keyboard::Modifier::SHIFT | keyboard::Modifier::CONTROL,
        keysym: sym,
        press: keyboard::Press::Press,
        value: Event::ClientUpdate(tag, Tag::Toggle(index)),
    })?;

    Ok(())
}

fn run(wm: &mut wm::WindowManager<Event>) -> Result<(), error::Error> {
    let mut tags = Tags::new();
    let mut tagsets: HashMap<MonitorId, Vec<TagSetId>> = HashMap::new();

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


    wm.bind(&keyboard::Binding {
        view: None,
        mask: keyboard::Modifier::MOD4,
        keysym: keysym::j,
        press: keyboard::Press::Press,
        value: Event::FocusNext,
    })?;

    wm.bind(&keyboard::Binding {
        view: None,
        mask: keyboard::Modifier::MOD4,
        keysym: keysym::k,
        press: keyboard::Press::Press,
        value: Event::FocusPrevious,
    })?;


    wm.flush()?;

    loop {
        match wm.next()? {
            wm::Event::MonitorConnect(id) => {
                let monitor = wm.get_mut(id).unwrap();
                let rect = monitor.rect();

                println!("connect monitor: {}", rect);

                let tagset = TagSet::new(vec!["a", "s", "d", "f", "g"]);
                let tagid = tags.insert(tagset);

                tagsets.entry(id).or_insert(vec![]).push(tagid);

                tag(wm, keysym::a, id, tagid, 0)?;
                tag(wm, keysym::s, id, tagid, 1)?;
                tag(wm, keysym::d, id, tagid, 2)?;
                tag(wm, keysym::f, id, tagid, 3)?;
                tag(wm, keysym::g, id, tagid, 4)?;

                wm.flush()?;
            }
            wm::Event::MonitorResize(view) => {
                if let Some(ids) = tagsets.get(&view) {
                    let selection = tags.select(ids);
                    wm.arrange(view, &selection)?;
                }
            }
            wm::Event::ClientCreate(mid, cid) => {
                wm.get_mut(mid)
                  .and_then(|mon| mon.get_mut(cid))
                  .map(|client| client.set_mask(tags.masks().clone()));

                if let Some(ids) = tagsets.get(&mid) {
                    let selection = tags.select(ids);
                    wm.arrange(mid, &selection)?;
                }
            }
            wm::Event::UserEvent(Event::Spawn(args)) => {
                wm.spawn(args);
            }
            wm::Event::UserEvent(Event::MonitorSet(view, tagset, tag)) => {
                tags.mask_mut(tagset).map(|mask| {
                    mask.clear();
                    mask.set(tag);
                });

                if let Some(ids) = tagsets.get(&view) {
                    let selection = tags.select(ids);
                    wm.arrange(view, &selection)?;
                }
            }
            wm::Event::UserEvent(Event::MonitorUpdate(view, tagset, tag)) => {
                tags.mask_mut(tagset).map(|mask| {
                    mask.set(tag);
                });

                if let Some(ids) = tagsets.get(&view) {
                    let selection = tags.select(ids);
                    wm.arrange(view, &selection)?;
                }
            }
            wm::Event::UserEvent(Event::ClientSet(tagset, tag)) => {
                wm.get_focus()
                  .and_then(|f| wm.get_mut(f))
                  .and_then(|v| v.get_mut(v.focus))
                  .and_then(|c| c.get_mask_mut(tagset))
                  .map(|t| {
                      t.clear();
                      t.set(tag);
                  });

                if let Some(view) = wm.get_focus() {
                    if let Some(ids) = tagsets.get(&view) {
                        let selection = tags.select(ids);
                        wm.arrange(view, &selection)?;
                    }
                }
            }
            wm::Event::UserEvent(Event::ClientUpdate(tagset, tag)) => {
                wm.get_focus()
                  .and_then(|focus| wm.get_mut(focus))
                  .and_then(|view| view.get_mut(view.focus))
                  .and_then(|client| client.get_mask_mut(tagset))
                  .map(|mask| mask.set(tag));

                if let Some(view) = wm.get_focus() {
                    if let Some(ids) = tagsets.get(&view) {
                        let selection = tags.select(ids);
                        wm.arrange(view, &selection)?;
                    }
                }
            }
            wm::Event::UserEvent(Event::FocusNext) => {
                wm.get_focus()
                    .and_then(|f| wm.get(f).map(|v| (f, v)))
                    .and_then(|(f, v)| v.next(v.focus).map(|c| (f, dbg!(c))))
                    .map(|(f, v)| wm.set_focus(f, v));
            }
            wm::Event::UserEvent(Event::FocusPrevious) => {
                wm.get_focus()
                    .and_then(|f| wm.get(f).map(|m| (f, m)))
                    .and_then(|(f, m)| m.previous(m.focus).map(|c| (f, dbg!(c))))
                    .map(|(f, m)| wm.set_focus(f, m));
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
