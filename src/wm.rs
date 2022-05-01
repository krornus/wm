use std::sync::Arc;
use std::os::unix::prelude::{AsRawFd, RawFd};
use std::sync::atomic::{AtomicUsize, Ordering};

use fork::Fork;
use signal_hook::consts::signal::*;
use xcb::x::{self, Keysym, Keycode};

use crate::error::Error;
use crate::kb::KeyManager;

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect { x, y, w, h }
    }

    pub fn center_x(&self) -> usize {
        self.x + (self.w / 2)
    }
}

pub trait Layout {
    fn arrange(&mut self, scope: &Rect, count: usize, index: usize) -> Rect;
}

pub struct LeftMaster { }

impl Layout for LeftMaster {

    fn arrange(&mut self, scope: &Rect, count: usize, index: usize) -> Rect {
        if index == 0 {
            if count == 1 {
                Rect::new(0, 0, scope.w, scope.h)
            } else {
                Rect::new(0, 0, scope.center_x(), scope.h)
            }
        } else {
            /* height of one box */
            let boxh = scope.h / (count - 1);
            /* pos of one box */
            let posh = boxh * (index - 1);

            Rect::new(scope.center_x(), posh, scope.w, posh + boxh)
        }
    }
}

pub struct Client {
    scope: Rect,
    window: x::Window,
}

impl Client {
    fn new(window: x::Window) -> Self {
        Client {
            scope: Rect::new(0, 0, 1, 1),
            window: window,
        }
    }

    fn resize(&mut self, adapter: &mut Adapter, scope: Rect) -> xcb::VoidCookieChecked {
        self.scope = scope;

        adapter.conn.send_request_checked(&x::ConfigureWindow {
            window: self.window,
            value_list: &[
                x::ConfigWindow::X(self.scope.x as i32),
                x::ConfigWindow::Y(self.scope.y as i32),
                x::ConfigWindow::Width(self.scope.w as u32),
                x::ConfigWindow::Height(self.scope.h as u32),
            ],
        })
    }
}

pub struct Monitor {
    scope: Rect,
    root: x::Window,
    clients: Vec<Client>,
    layout: Box<dyn Layout>,
}

impl Monitor {
    fn new<T: 'static + Layout>(scr: &x::Screen, layout: T) -> Self {
        let scope = Rect::new(
            0, 0,
            scr.width_in_pixels() as usize,
            scr.height_in_pixels() as usize,
        );

        Monitor {
            scope: scope,
            root: scr.root(),
            clients: vec![],
            layout: Box::new(layout),
        }
    }

    fn client(&self, window: x::Window) -> Option<&Client> {
        self.clients.iter().find(|x| x.window == window)
    }

    fn client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.clients.iter_mut().find(|x| x.window == window)
    }

    fn arrange(&mut self, adapter: &mut Adapter) -> Result<(), Error> {
        let count = self.clients.len();
        let mut cookies = Vec::with_capacity(count);

        for (i, client) in self.clients.iter_mut().enumerate() {
            let scope = self.layout.arrange(&self.scope, count, i);
            cookies.push(client.resize(adapter, scope));
        }

        for c in cookies {
            adapter.conn.check_request(c)?;
        }

        Ok(())
    }
}

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
    selmon: usize,
    monitors: Vec<Monitor>,
    signal: Arc<AtomicUsize>,
}

pub enum Event<T> {
    Empty,
    Interrupt,
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
        conn.send_and_check_request(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::STRUCTURE_NOTIFY |
                x::EventMask::PROPERTY_CHANGE |
                x::EventMask::SUBSTRUCTURE_NOTIFY |
                x::EventMask::SUBSTRUCTURE_REDIRECT
            )],
        }).map_err(|_| Error::AlreadyRunning)?;

        let keys = KeyManager::new(&conn)?;
        let monitors = setup.roots().map(|x| {
            Monitor::new(x, LeftMaster {})
        }).collect();

        let wm = WindowManager {
            keys: keys,
            selmon: 0,
            monitors: monitors,
            signal: Arc::new(AtomicUsize::new(0)),
            adapter: Adapter::new(conn, root),
        };

        signal_hook::flag::register_usize(SIGCHLD, Arc::clone(&wm.signal), SIGCHLD as usize)
            .map_err(|e| Error::SignalError(e))?;
        signal_hook::flag::register_usize(SIGINT, Arc::clone(&wm.signal), SIGINT as usize)
            .map_err(|e| Error::SignalError(e))?;

        Self::reap()?;

        Ok(wm)
    }

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
        /* xcb opens its descriptors with CLOEXEC */
        if let Some(args) = shlex::split(cmd) {
            if let Ok(Fork::Child) = fork::fork() {
                fork::setsid().expect("setsid failed");

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

        self.adapter.conn.send_and_check_request(&x::ConfigureWindow {
            window: event.window(),
            value_list: values.as_slice(),
        })?;

        Ok(())
    }

    fn map(&mut self, e: &x::MapRequestEvent) -> Result<(), Error> {
        self.monitors[self.selmon].clients.push(Client::new(e.window()));
        self.monitors[self.selmon].arrange(&mut self.adapter)?;

        self.adapter.conn.send_and_check_request(&x::MapWindow {
            window: e.window()
        })?;

        Ok(())
    }

    fn client(&self, window: x::Window) -> Option<&Client> {
        self.monitors.iter()
            .find_map(|x| x.client(window))
    }

    fn client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.monitors.iter_mut()
            .find_map(|x| x.client_mut(window))
    }

    fn reap() -> Result<bool, Error> {
        let mut zombie = false;

        loop {
            let rv =
                unsafe { libc::waitpid(-1, std::ptr::null::<*const i32>() as *mut i32, libc::WNOHANG) };

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

impl<T: Copy> AsRawFd for WindowManager<T> {
    /* for use with epoll etc... */
    fn as_raw_fd(&self) -> RawFd {
        self.adapter.conn.as_raw_fd()
    }
}
