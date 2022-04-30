use clap::Parser;
use xcb::x::KeyButMask;

mod kb;
mod wm;
mod error;

use crate::error::Error;
use crate::kb::keysym;

/// Rust window manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}

#[derive(Clone, Copy, Debug)]
enum Commands {
    Exit,
    Execute(&'static str),
}

fn run(conn: &mut wm::WindowManager<Commands>) -> Result<(), Error> {
    conn.bind(KeyButMask::MOD4, keysym::q, Commands::Exit)?;
    conn.bind(KeyButMask::MOD4, keysym::d, Commands::Execute("rofi -show run"))?;
    conn.bind(KeyButMask::MOD4, keysym::Return, Commands::Execute("xterm"))?;

    loop {
        match conn.next()? {
            wm::Event::UserEvent(Commands::Exit) => break,
            wm::Event::UserEvent(Commands::Execute(s)) => {
                conn.spawn(s);
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

    run(&mut conn).expect("window manager error");
}
