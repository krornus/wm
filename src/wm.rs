use std::os::unix::prelude::{AsRawFd, RawFd};
use std::process::Command;

use fork::Fork;
use xcb::x::{self, Keysym, Keycode};

use crate::error::Error;
use crate::kb::KeyManager;

pub struct Adapter {
    pub root: x::Window,
    pub conn: xcb::Connection,
}

impl Adapter {
    pub fn new(conn: xcb::Connection, root: x::Window) -> Self {
        Adapter {
            root: root,
            conn: conn,
        }
    }
}

pub struct WindowManager<T: Copy> {
    adapter: Adapter,
    keys: KeyManager<T>,
}

pub enum Event<T> {
    Empty,
    UserEvent(T),
}

impl<T: Copy> WindowManager<T> {
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect(name)?;

        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let root = screen.root();
        let cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::STRUCTURE_NOTIFY |
                x::EventMask::PROPERTY_CHANGE |
                x::EventMask::SUBSTRUCTURE_NOTIFY |
                x::EventMask::SUBSTRUCTURE_REDIRECT
            )],
        });

        conn.check_request(cookie).map_err(|_| Error::AlreadyRunning)?;

        let keys = KeyManager::new(&conn)?;
        let adapter = Adapter::new(conn, root);

        let wm = WindowManager {
            keys: keys,
            adapter: adapter,
        };

        Ok(wm)
    }

    pub fn next(&mut self) -> Result<Event<T>, Error> {
        let event = self.adapter.conn.wait_for_event()?;

        let ret = match event {
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                let value = self.keys.get(e.state(), e.detail() as Keycode);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            }
            xcb::Event::X(xcb::x::Event::MapRequest(ref e)) => {
                self.map(e)?;
                Ok(Event::Empty)
            },
            xcb::Event::X(xcb::x::Event::ConfigureRequest(ref e)) => {
                self.configure(e)?;
                Ok(Event::Empty)
            },
            _ => {
                Ok(Event::Empty)
            },
        };

        ret
    }

    pub fn spawn(&self, cmd: &str) {
        if let Some(args) = shlex::split(cmd) {
            if let Ok(Fork::Child) = fork::fork() {
                fork::setsid().expect("setsid failed");

                let cs: Vec<_> = args.into_iter().map(|x| {
                    std::ffi::CString::new(x)
                        .expect("spawn: invalid arguments")
                        .into_raw()
                }).collect();

                unsafe {
                    libc::execvp(cs[0], (&cs[..]).as_ptr() as *const *const i8);
                }
                unreachable!();
            }
        }
    }

    pub fn bind(&mut self, m: x::KeyButMask, k: Keysym, v: T) -> Result<(), Error> {
        self.keys.bind(&mut self.adapter, m, k, v)
    }
}

impl<T: Copy> WindowManager<T> {
    fn configure(&mut self, event: &x::ConfigureRequestEvent) -> Result<(), Error> {
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

        let cfg = x::ConfigureWindow {
            window: event.window(),
            value_list: values.as_slice(),
        };

        let cookie = self.adapter.conn.send_request_checked(&cfg);
        self.adapter.conn.check_request(cookie)?;

        Ok(())
    }

    fn map(&mut self, e: &x::MapRequestEvent) -> Result<(), Error> {
        let cookie = self.adapter.conn.send_request_checked(&x::MapWindow {
            window: e.window()
        });

        self.adapter.conn.check_request(cookie)?;

        Ok(())
    }
}

impl<T: Copy> AsRawFd for WindowManager<T> {
    /* for use with epoll etc... */
    fn as_raw_fd(&self) -> RawFd {
        self.adapter.conn.as_raw_fd()
    }
}
