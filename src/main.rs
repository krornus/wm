#![allow(dead_code)]
use xcb::x;

mod wm;
mod kb;
mod tag;
mod slab;
mod tree;
mod client;
mod display;
mod window;
mod layout;
mod rect;
mod error;

#[derive(Copy, Clone)]
enum Event {
    Exit,
    Spawn(&'static str),
}

fn run(wm: &mut wm::WindowManager<Event>) -> Result<(), error::Error> {
    wm.bind(&kb::Binding {
        view: None,
        mask: x::KeyButMask::MOD4,
        keysym: kb::keysym::Return,
        press: kb::Press::Press,
        value: Event::Spawn("sakura"),
    })?;

    wm.bind(&kb::Binding {
        view: None,
        mask: x::KeyButMask::MOD4,
        keysym: kb::keysym::q,
        press: kb::Press::Press,
        value: Event::Exit,
    })?;

    wm.flush()?;

    loop {
        match wm.next()? {
            wm::Event::MonitorConnect(id) => {
                let rect = wm.display().get_view(id).unwrap().rect();
                println!("connect monitor: {}", rect);
            },
            wm::Event::MonitorResize(id) => {
                wm.arrange(id)?;
            },
            wm::Event::UserEvent(Event::Spawn(args)) => {
                wm.spawn(args);
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
