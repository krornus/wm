#![allow(dead_code)]
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

}

fn run(wm: &mut wm::WindowManager<Event>) -> Result<(), error::Error> {
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");
    wm.spawn("xterm /bin/zsh");

    loop {
        match wm.next()? {
            wm::Event::MonitorConnect(_) => {
                println!("connect monitor");
            },
            wm::Event::MonitorResize(id) => {
                println!("resize monitor");
                wm.arrange(id)?;
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
