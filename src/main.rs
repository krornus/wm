#![allow(dead_code)]
mod wm;
mod kb;
mod tag;
mod client;
mod monitor;
mod container;
mod layout;
mod rect;
mod error;

#[derive(Copy, Clone)]
enum Event {

}

fn main() {
    let mut conn = wm::WindowManager::<Event>::connect(None)
        .expect("failed to connect to X11 server");
}
