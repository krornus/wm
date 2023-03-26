mod slab;
mod tree;
mod rect;
mod error;

mod monitor;
mod keyboard;
mod keysym;
mod client;

mod tag;
mod pane;
mod layout;

mod comm;
mod manager;

fn run() -> Result<(), error::Error> {
    let mut conn = manager::Connection::connect(None)?;
    let mut mgr = manager::WindowManager::new(&mut conn)?;

    loop {
        mgr.consume(&mut conn)?;

        if let Some(event) = conn.pop() {
            println!("{:?}", event);
        }
    }
}

fn main() {
    run().expect("window manager error");
}
