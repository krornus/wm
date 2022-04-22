use clap::Parser;
use xcb::x::KeyButMask;

mod wm;
mod kb;
mod ffi;
mod rect;
mod layout;
mod error;

use crate::error::Error;
use crate::kb::keysym;

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

fn run(conn: &mut wm::WindowManager<Commands>) -> Result<(), Error> {
    let mut binder = conn.binder()?;
    binder.add(KeyButMask::MOD4, keysym::q, Commands::Exit);
    binder.add(KeyButMask::MOD4, keysym::d, Commands::Execute("rofi -show run"));
    binder.add(KeyButMask::MOD4, keysym::Return, Commands::Execute("sakura"));
    binder.bind()?;

    loop {
        match conn.next()? {
            wm::Event::UserEvent(Commands::Exit) => break,
            wm::Event::UserEvent(Commands::Execute(s)) => conn.spawn(s)?,
            wm::Event::Map(w) => {
                conn.map(w);
                conn.sync()?;
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
