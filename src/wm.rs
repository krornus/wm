use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::error::Error;
use crate::rect::Rect;
use crate::kb::KeyManager;
use crate::tag::TagManager;
use crate::monitor::Display;
use crate::client::Client;


use fork::Fork;
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
    display: Display,
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
        let keys = KeyManager::new(&conn, root)?;
        let display = Display::new(&conn, root)?;

        let wm = WindowManager {
            signal: Arc::new(AtomicUsize::new(0)),
            adapter: Adapter::new(conn),
            keys: keys,
            tags: tags,
            display: display,
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

        // println!("event: {:?}", event);

        match event {
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                let value = self.keys.get(e.state(), e.detail() as Keycode);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            },
            xcb::Event::X(xcb::x::Event::ConfigureRequest(ref e)) => {
                self.configure(e)
            },
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

    pub fn spawn(&self, cmd: &str) {
        /* xcb opens its descriptors with CLOEXEC */
        if let Some(args) = shlex::split(cmd) {
            if let Ok(Fork::Child) = fork::fork() {
                fork::setsid().expect("setsid failed");

                println!("{:?}", args);

                /* swap to const pointers. into_raw() can leak here
                 * because we will execvp() or unreachable!() */
                let cs: Vec<_> = args.into_iter().map(|x| {
                    std::ffi::CString::new(x)
                        .expect("spawn: invalid arguments")
                        .into_raw()
                }).collect();

                unsafe {
                    libc::execvp(cs[0], (&cs[..]).as_ptr() as *const *const i8);
                }

                eprintln!("failed to spawn process");
                std::process::exit(1);
            }
        }
    }

    /// handle a ConfigureRequestEvent, which is a request to configure a window's properties
    fn configure(&mut self, event: &x::ConfigureRequestEvent) -> Result<Event<T>, Error> {
        let mask = event.value_mask();
        let mut values = Vec::with_capacity(7);

        if mask.contains(xcb::x::ConfigWindowMask::X) {
            values.push(x::ConfigWindow::X(event.x() as i32));
        }

        if mask.contains(xcb::x::ConfigWindowMask::Y) {
            values.push(x::ConfigWindow::Y(event.y() as i32));
        }

        if mask.contains(xcb::x::ConfigWindowMask::WIDTH) {
            values.push(x::ConfigWindow::Width(event.width() as u32));
        }

        if mask.contains(xcb::x::ConfigWindowMask::HEIGHT) {
            values.push(x::ConfigWindow::Height(event.height() as u32));
        }

        if mask.contains(xcb::x::ConfigWindowMask::BORDER_WIDTH) {
            values.push(x::ConfigWindow::BorderWidth(event.border_width() as u32));
        }

        if mask.contains(xcb::x::ConfigWindowMask::SIBLING) {
            values.push(x::ConfigWindow::Sibling(event.sibling()));
        }

        if mask.contains(xcb::x::ConfigWindowMask::STACK_MODE) {
            values.push(x::ConfigWindow::StackMode(event.stack_mode()));
        }

        /* TODO: mask checking is breaking */
        let rect = Rect::new(event.x(), event.y(), event.width(), event.height());

        println!("configure new window: {:?} - {}", event.window(), rect);
        self.display.client(event.window(), rect)?;

        self.adapter.conn.send_and_check_request(&x::ConfigureWindow {
            window: event.window(),
            value_list: values.as_slice(),
        })?;

        Ok(Event::Empty)
    }

    /// handle the MapRequestEvent, which is a request for us to show a window on screen
    fn map(&mut self, e: &x::MapRequestEvent) -> Result<Event<T>, Error> {
        println!("map window: {:?}", e.window());
        self.display.client(e.window(), Rect::new(0, 0, 0, 0))?;
        Ok(Event::Empty)
    }

    /// handle the UnmapNotifyEvent, which notifies us that a window has been unmapped (hidden)
    fn unmap(&mut self, _: &x::UnmapNotifyEvent) -> Result<Event<T>, Error> {
        Ok(Event::Empty)
    }

    /// handle the DestroyNotify, which notifies us that a window has been destroyed
    fn destroy(&mut self, _: &x::DestroyNotifyEvent) -> Result<Event<T>, Error> {
        Ok(Event::Empty)
    }
}
