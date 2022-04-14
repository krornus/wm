use std::ptr;
use std::sync::Arc;
use std::process::Command;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::os::unix::prelude::{AsRawFd, RawFd};

use fork::{Fork};
use xcb::{self, x};
use xkbcommon::xkb::Keysym;
use signal_hook::consts::signal::*;

use crate::error::Error;
use crate::keyboard::Keyboard;

pub struct WindowManager {
    conn: xcb::Connection,
    root: x::Window,
    sigchld: Arc<AtomicUsize>,
    sigint: Arc<AtomicUsize>,
    keyboard: Keyboard,
}

pub enum Event {
    KeyPress(x::KeyButMask, Keysym),
    Map(x::Window),
}

impl WindowManager {
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect(name)?;

        let setup = conn.get_setup();
        let scr = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let root = scr.root();
        let keyboard = Keyboard::new(&conn)?;

        let mut wm = WindowManager {
            conn: conn,
            root: root,
            sigchld: Arc::new(AtomicUsize::new(0)),
            sigint: Arc::new(AtomicUsize::new(0)),
            keyboard: keyboard,
        };

        signal_hook::flag::register_usize(SIGCHLD, Arc::clone(&wm.sigchld), SIGCHLD as usize)
            .map_err(|e| Error::SignalError(e))?;

        signal_hook::flag::register_usize(SIGINT, Arc::clone(&wm.sigint), SIGINT as usize)
            .map_err(|e| Error::SignalError(e))?;

        wm.lock()?;
        wm.select()?;

        Self::reap()?;

        Ok(wm)
    }

    pub fn next(&mut self) -> Result<Option<Event>, Error> {
        let event = self.conn.wait_for_event()?;

        match event {
            xcb::Event::Xkb(xcb::xkb::Event::StateNotify(ref e)) => {
                self.keyboard.update_mask(e);
                Ok(None)
            },
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                Ok(Some(Event::KeyPress(e.state(), self.keyboard.keysym(e))))
            },
            xcb::Event::X(xcb::x::Event::MapRequest(e)) => {
                Ok(Some(Event::Map(e.window())))
            },
            _ => Ok(None),
        }
    }

    pub fn spawn(&self, cmd: &str) -> Result<(), Error> {
        if let Some(args) = shlex::split(cmd) {
            if let Ok(Fork::Child) = fork::fork() {
                fork::setsid()
                    .expect("setsid failed");

                Command::new(&args[0])
                    .args(&args[1..])
                    .spawn()
                    .expect(&format!("process failed: {}", cmd));

            }
        }

        Ok(())
    }

    fn reap() -> Result<bool, Error> {
        let mut zombie = false;

        loop {
            let rv =
                unsafe { libc::waitpid(-1, ptr::null::<*const i32>() as *mut i32, libc::WNOHANG) };

            if rv < 0 {
                let e = std::io::Error::last_os_error();
                let errno = std::io::Error::raw_os_error(&e);
                match errno {
                    Some(libc::ECHILD) => break Ok(zombie),
                    Some(_) => break Err(Error::IoError(e)),
                    None => unreachable!(),
                }
            } else if rv == 0 {
                break Ok(zombie);
            } else {
                zombie = true;
            }
        }
    }

    fn lock(&mut self) -> Result<(), Error> {
        let cookie = self.conn.send_request_checked(&x::ChangeWindowAttributes {
            window: self.root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::SUBSTRUCTURE_REDIRECT
            )],
        });

        self.conn
            .check_request(cookie)
            .map_err(|_| Error::AlreadyRunning)
    }

    fn select(&mut self) -> Result<(), Error> {
        let cookie = self.conn.send_request_checked(&x::ChangeWindowAttributes {
            window: self.root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::SUBSTRUCTURE_REDIRECT |
                x::EventMask::KEY_PRESS,
            )],
        });

        self.conn.check_request(cookie)?;
        Ok(())
    }

    pub fn map(&mut self, win: x::Window) -> Result<(), Error> {
        let a = self.conn.send_request_checked(&x::ChangeWindowAttributes {
            window: win,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::SUBSTRUCTURE_REDIRECT |
                x::EventMask::KEY_PRESS,
            )],
        });

        let b = self.conn.send_request_checked(&x::MapWindow {
            window: win,
        });

        self.conn.check_request(a)?;
        self.conn.check_request(b)?;

        Ok(())
    }
}

impl AsRawFd for WindowManager {
    fn as_raw_fd(&self) -> RawFd {
        self.conn.as_raw_fd()
    }
}

pub struct KeyManager<T> {
    map: HashMap<(x::KeyButMask, Keysym), T>,
}

impl<T: Copy> KeyManager<T> {
    pub fn new() -> Self {
        KeyManager {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, m: x::KeyButMask, k: Keysym, v: T) {
        self.map.insert((m, k), v);
    }

    pub fn get(&self, m: x::KeyButMask, k: Keysym) -> Option<T> {
        self.map.get(&(m, k)).copied()
    }
}
