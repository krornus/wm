use clap::Parser;

mod wm;

/// Rust window manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
}

fn main() {
    let args = Args::parse();
    let mut conn = wm::WindowManager::connect(None)
        .expect("failed to connect to X11 server");

    conn.run()
        .expect("window manager error");
}
