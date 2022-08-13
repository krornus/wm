#![allow(dead_code)]
mod wm;
mod kb;
mod tag;
mod slab;
mod tree;
mod client;
mod display;
mod container;
mod layout;
mod rect;
mod error;

#[derive(Copy, Clone)]
enum Event {

}

fn run(wm: &mut wm::WindowManager<Event>) -> Result<(), error::Error> {
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");

    loop {
        wm.next()?;
    }

}

fn main() {
    let mut wm = wm::WindowManager::<Event>::connect(None)
        .expect("failed to connect to X11 server");

    run(&mut wm).expect("window manager error");
}
