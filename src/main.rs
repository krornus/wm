use clap::Parser;
use xcb::x::KeyButMask;
use xkbcommon::xkb::keysyms::*;

mod wm;
mod error;
mod keyboard;

use crate::error::Error;

/// https://www.x.org/releases/X11R7.7/doc/libxcb/tutorial/index.html#wm
/// https://jichu4n.com/posts/how-x-window-managers-work-and-how-to-write-one-part-iii/

/// Rust window manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}

#[derive(Clone, Copy)]
enum Commands {
    Exit,
    Execute(&'static str),
}

fn run(conn: &mut wm::WindowManager) -> Result<(), Error> {

    let mut km = wm::KeyManager::new();
    km.add(KeyButMask::MOD4, KEY_q, Commands::Exit);
    km.add(KeyButMask::MOD4, KEY_d, Commands::Execute("rofi -show run"));
    km.add(KeyButMask::MOD4, KEY_Return, Commands::Execute("sakura"));

    loop {
        match conn.next()? {
            Some(wm::Event::KeyPress(m, k)) => {
                match km.get(m, k) {
                    Some(Commands::Exit) => break,
                    Some(Commands::Execute(s)) => conn.spawn(s)?,
                    None => continue,
                }
            },
            Some(wm::Event::Map(w)) => {
                conn.map(w)?
            },
            _ => continue,
        }
    }

    Ok(())
}

fn main() {
    let _args = Args::parse();
    let mut conn = wm::WindowManager::connect(None)
        .expect("failed to connect to X11 server");

    run(&mut conn)
        .expect("window manager error");
}
