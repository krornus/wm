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
    View(usize),
    Select(usize),
    Focus(Option<isize>),
    Execute(&'static str),
}

fn run(conn: &mut wm::WindowManager<Commands>) -> Result<(), Error> {
    let global = conn.tagset(vec!["main"]);

    // let mut tag = wm::TagSelection {
    //     scope: wm::TagScope::Monitor,
    //     index: 0,
    //     mask: 1,
    // };

    for m in conn.monitors_mut() {
        let id = conn.tagset(vec!["a", "s", "d", "f"]);
    }

    conn.bind(KeyButMask::MOD4, keysym::a, Commands::View(0))?;
    conn.bind(KeyButMask::MOD4, keysym::s, Commands::View(1))?;
    conn.bind(KeyButMask::MOD4, keysym::d, Commands::View(2))?;
    conn.bind(KeyButMask::MOD4, keysym::f, Commands::View(3))?;

    conn.bind(KeyButMask::MOD4 | KeyButMask::SHIFT, keysym::a, Commands::Select(0))?;
    conn.bind(KeyButMask::MOD4 | KeyButMask::SHIFT, keysym::s, Commands::Select(1))?;
    conn.bind(KeyButMask::MOD4 | KeyButMask::SHIFT, keysym::d, Commands::Select(2))?;
    conn.bind(KeyButMask::MOD4 | KeyButMask::SHIFT, keysym::f, Commands::Select(3))?;

    conn.bind(KeyButMask::MOD4, keysym::q, Commands::Exit)?;
    conn.bind(KeyButMask::MOD4, keysym::d, Commands::Execute("rofi -show run"))?;
    conn.bind(KeyButMask::MOD4, keysym::Return, Commands::Execute("xterm"))?;
    conn.bind(KeyButMask::MOD4, keysym::j, Commands::Focus(Some(1)))?;
    conn.bind(KeyButMask::MOD4, keysym::k, Commands::Focus(Some(-1)))?;

    loop {
        match conn.next()? {
            wm::Event::UserEvent(Commands::Exit) => break,
            wm::Event::UserEvent(Commands::Focus(x)) => {
                conn.set_focus(x)?;
            },
            wm::Event::UserEvent(Commands::Execute(s)) => {
                conn.spawn(s);
            },
            wm::Event::UserEvent(Commands::View(n)) => {
                // conn.view(n)?;
            },
            wm::Event::UserEvent(Commands::Select(n)) => {
                // tag.mask = 1 << n;
                // conn.focus_mut().map(|c| c.select(&tag))?;
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
