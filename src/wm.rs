use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::client::Client;
use crate::display::{Display, Monitor, MonitorId};
use crate::error::Error;
use crate::keyboard::{Binding, Keys};
use crate::rect::Rect;
use crate::tag::{TagSelection, Tags};

use fork::Fork;
use signal_hook::consts::signal::*;
use xcb::randr;
use xcb::x::{self, Keycode};

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
        let cookie = self.conn.send_request_checked(request);
        self.pending.push(cookie);
    }

    pub fn flush(&mut self) -> Result<bool, Error> {
        let ok = self.pending.len() > 0;

        for c in self.pending.drain(..) {
            self.conn.check_request(c)?;
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
    MonitorConnect(MonitorId),
    MonitorResize(MonitorId),
    MonitorDisconnect(MonitorId),
    MonitorPrimary(MonitorId),
    ClientCreate(MonitorId, usize),
    ClientDestroy(usize),
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
        let (conn, main) =
            xcb::Connection::connect_with_extensions(name, &[xcb::Extension::RandR], &[])?;

        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let root = screen.root();

        conn.send_and_check_request(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::STRUCTURE_NOTIFY
                    | x::EventMask::PROPERTY_CHANGE
                    | x::EventMask::SUBSTRUCTURE_NOTIFY
                    | x::EventMask::SUBSTRUCTURE_REDIRECT,
            )],
        })
        .map_err(|_| Error::AlreadyRunning)?;

        let mut adapter = Adapter::new(conn);

        let tags = Tags::new();
        let keys = Keys::new(&adapter.conn, root)?;
        let display = Display::new(&mut adapter, root)?;

        adapter.conn.send_and_check_request(&randr::SelectInput {
            window: root,
            enable: randr::NotifyMask::SCREEN_CHANGE
                | randr::NotifyMask::OUTPUT_CHANGE
                | randr::NotifyMask::CRTC_CHANGE
                | randr::NotifyMask::OUTPUT_PROPERTY,
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
                    -1,
                    std::ptr::null::<*const i32>() as *mut i32,
                    libc::WNOHANG,
                )
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

        match self.adapter.pop() {
            Some(e) => return Ok(e),
            None => {}
        }

        let e = match event {
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                let focus = self.display.focus;
                let value = self.keys.get(focus, e.state(), e.detail() as Keycode, true);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            }
            xcb::Event::X(xcb::x::Event::KeyRelease(ref e)) => {
                let focus = self.display.focus;
                let value = self
                    .keys
                    .get(focus, e.state(), e.detail() as Keycode, false);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            }
            xcb::Event::X(xcb::x::Event::ConfigureRequest(ref e)) => self.configure(e),
            xcb::Event::X(xcb::x::Event::MapRequest(ref e)) => self.map(e),
            xcb::Event::X(xcb::x::Event::UnmapNotify(ref e)) => self.unmap(e),
            xcb::Event::X(xcb::x::Event::DestroyNotify(ref e)) => self.destroy(e),
            xcb::Event::RandR(xcb::randr::Event::ScreenChangeNotify(_)) => {
                self.display.update(&mut self.adapter)?;
                Ok(self.adapter.pop().unwrap_or(Event::Empty))
            }
            xcb::Event::X(xcb::x::Event::ConfigureNotify(ref e)) => {
                if self.root == e.window() {
                    /* this forces randr to update, flushing out any ScreenChangeNotifys */
                    self.display.reconfigure(&mut self.adapter, e.window())?;
                }

                Ok(Event::Empty)
            }
            xcb::Event::X(xcb::x::Event::EnterNotify(ref e)) => {
                self.display
                    .iter()
                    .find_map(|(vid, view)| view.find(e.event()).map(|cid| (vid, cid)))
                    .map(|(id, cid)| {
                        let vid = MonitorId::from(id);
                        self.display.focus = Some(vid);
                        self.display.get_mut(vid).map(|view| view.focus = cid)
                    });

                Ok(Event::Empty)
            }
            _ => Ok(Event::Empty),
        };

        self.adapter.flush()?;

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
                let cs: Vec<_> = args
                    .into_iter()
                    .map(|x| {
                        std::ffi::CString::new(x)
                            .expect("spawn: invalid arguments")
                            .into_raw()
                    })
                    .collect();

                unsafe {
                    libc::execvp(cs[0], (&cs[..]).as_ptr() as *const *const i8);
                }

                eprintln!("failed to spawn process");
                std::process::exit(1);
            }
        }
    }

    #[inline]
    pub fn get(&self, view: MonitorId) -> Option<&Monitor> {
        self.display.get(view)
    }

    #[inline]
    pub fn get_mut(&mut self, view: MonitorId) -> Option<&mut Monitor> {
        self.display.get_mut(view)
    }

    pub fn flush(&mut self) -> Result<bool, Error> {
        self.adapter.flush()
    }

    pub fn get_focus(&mut self) -> Option<MonitorId> {
        self.display.focus
    }

    pub fn set_focus(&mut self, mon: MonitorId, client: usize) {
        let a = &mut self.adapter;

        self.display
            .get_mut(mon)
            .map(|m| {
                match m.get_mut(client) {
                    Some(c) => {
                        c.focus(a);
                        m.focus = client;
                    }
                    None => {
                    }
                }
            });
    }

    pub fn arrange<'a, 'b>(
        &mut self,
        id: MonitorId,
        mask: &TagSelection<'a, 'b>,
    ) -> Result<(), Error> {
        match self.display.get_mut(id) {
            Some(view) => view.arrange(&mut self.adapter, mask),
            None => Ok(())
        }
    }
}

impl<T: Copy> WindowManager<T> {
    fn manage(&mut self, window: x::Window) -> Event<T> {
        let rect = Rect::new(0, 0, 0, 0);
        let client = Client::new(window, rect);
        let id = self.display.add(client);

        self.adapter.request(&x::ChangeWindowAttributes {
            window: window,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::ENTER_WINDOW | x::EventMask::FOCUS_CHANGE,
            )],
        });

        /* TODO: support empty output */
        Event::ClientCreate(self.display.focus.unwrap(), id)
    }

    /// handle a ConfigureRequestEvent, which is a request to configure a window's properties
    fn configure(&mut self, event: &x::ConfigureRequestEvent) -> Result<Event<T>, Error> {
        let mask = event.value_mask();
        let mut values = Vec::with_capacity(7);

        let client = self
            .display
            .iter()
            .find_map(|(_, view)| view.find(event.window()).and_then(|id| view.get(id)));

        let rect = if let Some(c) = client {
            *c.rect()
        } else {
            Rect::new(event.x(), event.y(), event.width(), event.height())
        };

        values.push(x::ConfigWindow::X(rect.x as i32));
        values.push(x::ConfigWindow::Y(rect.y as i32));
        values.push(x::ConfigWindow::Width(rect.w as u32));
        values.push(x::ConfigWindow::Height(rect.h as u32));

        values.push(x::ConfigWindow::BorderWidth(2));

        if mask.contains(xcb::x::ConfigWindowMask::SIBLING) {
            values.push(x::ConfigWindow::Sibling(event.sibling()));
        }

        if mask.contains(xcb::x::ConfigWindowMask::STACK_MODE) {
            values.push(x::ConfigWindow::StackMode(event.stack_mode()));
        }

        self.adapter
            .conn
            .send_and_check_request(&x::ConfigureWindow {
                window: event.window(),
                value_list: values.as_slice(),
            })?;

        Ok(Event::Empty)
    }

    /// handle the MapRequestEvent, which is a request for us to show a window on screen
    fn map(&mut self, e: &x::MapRequestEvent) -> Result<Event<T>, Error> {
        let client = self
            .display
            .iter()
            .find_map(|(_, view)| view.find(e.window()));

        if client.is_none() {
            Ok(self.manage(e.window()))
        } else {
            Ok(Event::Empty)
        }
    }

    /// handle the UnmapNotifyEvent, which notifies us that a window has been unmapped (hidden)
    fn unmap(&mut self, _: &x::UnmapNotifyEvent) -> Result<Event<T>, Error> {
        Ok(Event::Empty)
    }

    /// handle the DestroyNotify, which notifies us that a window has been destroyed
    fn destroy(&mut self, e: &x::DestroyNotifyEvent) -> Result<Event<T>, Error> {
        Ok(Event::Empty)
    }
}
