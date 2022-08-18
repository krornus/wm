use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::VecDeque;

use crate::error::Error;
use crate::rect::Rect;
use crate::kb::{Keys, Binding};
use crate::tag::Tags;
use crate::client::Client;
use crate::display::{ViewId, Display};

use fork::Fork;
use xcb::x::{self, Keycode};
use xcb::randr;
use signal_hook::consts::signal::*;

pub struct Adapter<T> {
    pub conn: xcb::Connection,
    pending: Vec<xcb::VoidCookieChecked>,
    events: VecDeque<Event<T>>,
}

impl<T> Adapter<T> {
    pub fn new(conn: xcb::Connection) -> Self {
        Adapter {
            conn: conn,
            pending: vec![],
            events: VecDeque::new(),
        }
    }

    pub fn request<R>(&mut self, request: &R)
    where
        R: xcb::RequestWithoutReply + std::fmt::Debug,
    {
        let cookie = self.conn.send_request_checked(dbg!(request));
        self.pending.push(cookie);
    }

    pub fn check(&mut self) -> Result<bool, Error> {
        let ok = self.pending.len() > 0;

        for c in self.pending.drain(..) {
            self.conn.check_request(dbg!(c))?;
        }

        Ok(ok)
    }

    pub fn push(&mut self, e: Event<T>) {
        self.events.push_front(e);
    }

    pub fn pop(&mut self) -> Option<Event<T>> {
        self.events.pop_back()
    }
}

pub enum Event<T> {
    Empty,
    Interrupt,
    MonitorConnect(ViewId),
    MonitorResize(ViewId),
    MonitorDisconnect(ViewId),
    MonitorPrimary(ViewId),
    UserEvent(T),
}

pub struct WindowManager<T: Copy> {
    root: x::Window,
    adapter: Adapter<T>,
    signal: Arc<AtomicUsize>,
    tags: Tags,
    display: Display,
    keys: Keys<T>,
}

impl<T: Copy> WindowManager<T> {
    /// Create a new WindowManager struct, with optional X11 display name
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect_with_extensions(
            name,
            &[xcb::Extension::RandR],
            &[]
        )?;

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

        let mut adapter = Adapter::new(conn);

        let tags = Tags::new();
        let keys = Keys::new(&adapter.conn, root)?;
        let display = Display::new(&mut adapter, root)?;

        adapter.conn.send_and_check_request(&randr::SelectInput {
            window: root,
            enable: randr::NotifyMask::SCREEN_CHANGE |
                    randr::NotifyMask::OUTPUT_CHANGE |
                    randr::NotifyMask::CRTC_CHANGE |
                    randr::NotifyMask::OUTPUT_PROPERTY
        })?;

        let wm = WindowManager {
            root: root,
            signal: Arc::new(AtomicUsize::new(0)),
            adapter: adapter,
            display: display,
            keys: keys,
            tags: tags,
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
	pub fn flush(&mut self) -> Result<bool, Error> {
		self.adapter.check()
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

        match self.adapter.pop() {
            Some(e) => { return Ok(e) },
            None => {},
        }

        let e = match event {
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                let focus = self.display.focus();
                let value = self.keys.get(focus, e.state(), e.detail() as Keycode, true);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            },
            xcb::Event::X(xcb::x::Event::KeyRelease(ref e)) => {
                let focus = self.display.focus();
                let value = self.keys.get(focus, e.state(), e.detail() as Keycode, false);
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
            xcb::Event::RandR(xcb::randr::Event::ScreenChangeNotify(_)) => {
                self.display.update(&mut self.adapter)?;
                Ok(Event::Empty)
            },
            xcb::Event::X(xcb::x::Event::ConfigureNotify(ref e)) => {
                if self.root == e.window() {
                    self.display.configure(&mut self.adapter, e.window())?;
                }
                Ok(Event::Empty)
            }
            _ => {
                Ok(Event::Empty)
            },
        };

        self.adapter.check()?;

        e
    }

    #[inline]
    pub fn bind(&mut self, binding: &Binding<T>) -> Result<(), Error> {
        self.keys.bind(&mut self.adapter, binding)
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

                eprintln!("failed to spawn process");
                std::process::exit(1);
            }
        }
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn display_mut(&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn arrange(&mut self, id: ViewId) -> Result<(), Error> {
        let view = self.display.get_view_mut(id).ok_or(Error::MissingView)?;

        view.arrange(&mut self.adapter);
        self.adapter.check()?;

        Ok(())
    }

    /// handle a ConfigureRequestEvent, which is a request to configure a window's properties
    fn configure(&mut self, event: &x::ConfigureRequestEvent) -> Result<Event<T>, Error> {
        let mask = event.value_mask();
        let mut values = Vec::with_capacity(7);

        let client = self.display.get_client(event.window());

        if let Some(c) = client {
            let rect = c.rect();
            values.push(x::ConfigWindow::X(rect.x as i32));
            values.push(x::ConfigWindow::Y(rect.y as i32));
            values.push(x::ConfigWindow::Width(rect.w as u32));
            values.push(x::ConfigWindow::Height(rect.h as u32));
        } else {
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

        Ok(Event::Empty)
    }

    /// handle the MapRequestEvent, which is a request for us to show a window on screen
    fn map(&mut self, e: &x::MapRequestEvent) -> Result<Event<T>, Error> {
        let client = Client::new(e.window(), Rect::new(0, 0, 0, 0));
        self.display.add_client(&mut self.adapter, client);

        match self.display.get_client_mut(e.window()) {
            Some(c) => c.show(&mut self.adapter, true),
            None => {},
        }

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
