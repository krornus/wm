use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::error::Error;
use crate::kb::KeyManager;
use crate::tag::TagManager;
use crate::monitor::MonitorManager;
use crate::container::Container;
use crate::client::Client;


use xcb::x::{self, Keycode};
use signal_hook::consts::signal::*;

pub struct Adapter {
    pub conn: xcb::Connection,
    pending: Vec<xcb::VoidCookieChecked>,
}

impl Adapter {
    pub fn new(conn: xcb::Connection) -> Self {
        Adapter {
            conn: conn,
            pending: vec![],
        }
    }

    pub fn request<R>(&mut self, request: &R)
    where
        R: xcb::RequestWithoutReply,
    {
        let cookie = self.conn.send_request_checked(request);
        self.pending.push(cookie);
    }

    pub fn check(&mut self) -> Result<bool, Error> {
        let ok = self.pending.len() > 0;
        for c in self.pending.drain(..) {
            self.conn.check_request(c)?;
        }

        Ok(ok)
    }
}

pub enum Event<'w, T> {
    Empty,
    Interrupt,
    ClientCreate(&'w mut Client),
    UserEvent(T),
}

pub struct WindowManager<T> {
    adapter: Adapter,
    signal: Arc<AtomicUsize>,
    tags: TagManager,
    keys: KeyManager<T>,
    monitors: MonitorManager,
}

impl<T: Copy> WindowManager<T> {
    /// Create a new WindowManager struct, with optional X11 display name
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect(name)?;

        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let root = screen.root();
        conn.send_and_check_request(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::STRUCTURE_NOTIFY |
                x::EventMask::PROPERTY_CHANGE |
                x::EventMask::SUBSTRUCTURE_NOTIFY |
                x::EventMask::SUBSTRUCTURE_REDIRECT
            )],
        }).map_err(|_| Error::AlreadyRunning)?;

        let tags = TagManager::new();
        let keys = KeyManager::new(&conn)?;
        let monitors = MonitorManager::new(&conn, root)?;

        let wm = WindowManager {
            keys: keys,
            monitors: monitors,
            tags: tags,
            signal: Arc::new(AtomicUsize::new(0)),
            adapter: Adapter::new(conn),
        };

        Ok(wm)
    }

    fn reap() -> Result<bool, Error> {
        let mut zombie = false;

        loop {
            let rv = unsafe {
                libc::waitpid(
                    -1, std::ptr::null::<*const i32>() as *mut i32,
                    libc::WNOHANG)
            };

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


impl<T: Copy> WindowManager<T> {
    pub fn next(&mut self) -> Result<Event<T>, Error> {
        let event = self.adapter.conn.wait_for_event()?;

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
            xcb::Event::X(xcb::x::Event::MapRequest(ref e)) => {
                self.map(e)
            },
            xcb::Event::X(xcb::x::Event::UnmapNotify(ref e)) => {
                self.unmap(e)
            },
            xcb::Event::X(xcb::x::Event::DestroyNotify(ref e)) => {
                self.destroy(e)
            },
            _ => {
                Ok(Event::Empty)
            },
        }
    }

    /// handle the MapRequestEvent, which is a request for us to show a window on screen
    fn map(&mut self, e: &x::MapRequestEvent) -> Result<Event<T>, Error> {
        Ok(Event::Empty)
    }

    /// handle the UnmapNotifyEvent, which notifies us that a window has been unmapped (hidden)
    fn unmap(&mut self, e: &x::UnmapNotifyEvent) -> Result<Event<T>, Error> {
        Ok(Event::Empty)
    }

    /// handle the DestroyNotify, which notifies us that a window has been destroyed
    fn destroy(&mut self, e: &x::DestroyNotifyEvent) -> Result<Event<T>, Error> {
        Ok(Event::Empty)
    }
}
