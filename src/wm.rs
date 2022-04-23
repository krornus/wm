use std::os::unix::prelude::{AsRawFd, RawFd};
use std::process::Command;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use fork::Fork;
use signal_hook::consts::signal::*;
use xcb::x::{self, Keysym, Keycode};

use crate::error::Error;
use crate::kb::KeyManager;
use crate::layout::{Layout, LeftMaster};
use crate::rect::Rect;

pub struct Monitor {
    area: Rect,
}

impl Monitor {
    fn new(screen: &x::Screen) -> Self {
        Monitor {
            area: Rect::new(
                0,
                0,
                screen.width_in_pixels() as usize,
                screen.height_in_pixels() as usize,
            ),
        }
    }
}

pub struct WindowManager<T> {
    root: x::Window,
    conn: xcb::Connection,
    pending: Vec<xcb::VoidCookieChecked>,
    signal: Arc<AtomicUsize>,
    keys: KeyManager<T>,
    layout: LeftMaster,
    monitors: Vec<Monitor>,
}

pub enum Event<T> {
    Empty,
    Interrupt,
    UserEvent(T),
    Map(x::Window),
}

impl<T: Copy + std::fmt::Debug> WindowManager<T> {
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect(name)?;

        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let monitors = setup.roots().map(|s| Monitor::new(s)).collect();

        let root = screen.root();
        let cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::SUBSTRUCTURE_REDIRECT,
            )],
        });

        conn.check_request(cookie)
            .map_err(|_| Error::AlreadyRunning)?;

        let keys = KeyManager::new(&conn)?;
        dbg!(0);

        let wm = WindowManager {
            conn: conn,
            root: root,
            signal: Arc::new(AtomicUsize::new(0)),
            keys: keys,
            pending: Vec::new(),
            monitors: monitors,
            layout: LeftMaster::new(),
        };

        signal_hook::flag::register_usize(SIGCHLD, Arc::clone(&wm.signal), SIGCHLD as usize)
            .map_err(|e| Error::SignalError(e))?;
        signal_hook::flag::register_usize(SIGINT, Arc::clone(&wm.signal), SIGINT as usize)
            .map_err(|e| Error::SignalError(e))?;

        dbg!(1);
        Self::reap()?;
        dbg!(2);

        Ok(wm)
    }

    pub fn next(&mut self) -> Result<Event<T>, Error> {
        self.sync()?;

        let event = self.conn.wait_for_event()?;

        const SIGCHLD_U: usize = SIGCHLD as usize;
        const SIGINT_U: usize = SIGINT as usize;

        match self.signal.load(Ordering::Relaxed) {
            SIGCHLD_U => {
                Self::reap()?;
            }
            SIGINT_U => {
                return Ok(Event::Interrupt);
            }
            _ => {}
        }

        match event {
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                let value = self.keys.get(e.state(), e.detail() as Keycode);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            }
            xcb::Event::X(xcb::x::Event::MapRequest(ref e)) => Ok(Event::Map(e.window())),
            xcb::Event::X(xcb::x::Event::ConfigureRequest(ref e)) => {
                self.configure(e);
                Ok(Event::Empty)
            }
            _ => Ok(Event::Empty),
        }
    }

    pub fn spawn(&self, cmd: &str) -> Result<(), Error> {
        if let Some(args) = shlex::split(cmd) {
            if let Ok(Fork::Child) = fork::fork() {
                fork::setsid().expect("setsid failed");

                Command::new(&args[0])
                    .args(&args[1..])
                    .spawn()
                    .expect(&format!("process failed: {}", cmd));
            }
        }

        Ok(())
    }

    pub fn bind(&mut self, m: x::KeyButMask, k: Keysym, v: T) -> Result<(), Error> {
        self.keys.bind(&self.conn, self.root, m, k, v)
    }

    pub fn map(&mut self, win: x::Window) {
        self.request(&x::ChangeWindowAttributes {
            window: win,
            value_list: &[xcb::x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_REDIRECT)],
        });

        self.request(&x::MapWindow { window: win });
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        /* is there a way to just scan for errors? */
        for cookie in self.pending.drain(..) {
            self.conn.check_request(cookie)?;
        }

        Ok(())
    }
}

impl<T> WindowManager<T> {
    fn request<R>(&mut self, req: &R)
    where
        R: xcb::RequestWithoutReply,
    {
        let cookie = self.conn.send_request_checked(req);
        self.pending.push(cookie);
    }

    fn configure(&mut self, e: &x::ConfigureRequestEvent) {
        let r = self.layout.arrange(&self.monitors[0].area);

        self.request(&x::ConfigureWindow {
            window: e.window(),
            value_list: &[
                x::ConfigWindow::X(r.x as i32),
                x::ConfigWindow::Y(r.y as i32),
                x::ConfigWindow::Width(r.w as u32),
                x::ConfigWindow::Height(r.h as u32),
            ],
        });
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
}

impl<T> AsRawFd for WindowManager<T> {
    fn as_raw_fd(&self) -> RawFd {
        self.conn.as_raw_fd()
    }
}
