use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::client::Client;
use crate::display::{Display, Monitor, MonitorId};
use crate::error::Error;
use crate::keyboard::{Binding, Keys};
use crate::rect::Rect;
use crate::tag::Tags;
use crate::painter::Painter;

use fork::Fork;
use signal_hook::consts::signal::*;
use xcb::randr;
use xcb::x::{self, Keycode};

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

pub struct Connection<T> {
    raw: xcb::Connection,
    screen: usize,
    root: x::Window,
    events: VecDeque<Event<T>>,
}

impl<T> Connection<T> {
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect_with_extensions(
            name,
            &[xcb::Extension::RandR],
            &[])?;

        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let root = screen.root();

        Ok(Connection {
            raw: conn,
            screen: main as usize,
            root: root,
            events: VecDeque::new(),
        })
    }

    #[inline]
    pub fn get_setup(&self) -> &x::Setup {
        self.raw.get_setup()
    }

    #[inline]
    pub fn generate_id<I: xcb::XidNew>(&self) -> I {
        self.raw.generate_id()
    }

    #[inline]
    #[must_use]
    pub fn send_request<R>(&self, req: &R) -> R::Cookie
    where
        R: xcb::Request,
    {
        self.raw.send_request(req)
    }

    #[inline]
    pub fn wait_for_reply<C>(&self, cookie: C) -> Result<C::Reply, Error>
    where
        C: xcb::CookieWithReplyChecked,
    {
        Ok(self.raw.wait_for_reply(cookie)?)
    }

    #[inline]
    #[must_use]
    pub fn send_request_checked<R>(&mut self, request: &R) -> xcb::VoidCookieChecked
    where
        R: xcb::RequestWithoutReply,
    {
        self.raw.send_request_checked(request)
    }

    pub fn send_and_check_request<R>(&self, req: &R) -> xcb::ProtocolResult<()>
    where
        R: xcb::RequestWithoutReply,
    {
        self.raw.send_and_check_request(req)
    }

    #[inline]
    pub fn check_request(&self, cookie: xcb::VoidCookieChecked) -> xcb::ProtocolResult<()> {
        self.raw.check_request(cookie)
    }


    #[inline]
    pub fn push(&mut self, e: Event<T>) {
        self.events.push_front(e);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Event<T>> {
        self.events.pop_back()
    }
}

pub struct WindowManager<T: Copy> {
    signal: Arc<AtomicUsize>,
    painter: Painter,
    tags: Tags,
    display: Display,
    keys: Keys<T>,
}

impl<T: Copy> WindowManager<T> {
    /// Create a new WindowManager struct, with optional X11 display name
    pub fn new(conn: &mut Connection<T>) -> Result<Self, Error> {
        let setup = conn.raw.get_setup();
        let screen = setup
            .roots()
            .nth(conn.screen)
            .ok_or(Error::MissingScreen)?;

        let root = screen.root();

        conn.raw.send_and_check_request(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::STRUCTURE_NOTIFY
                    | x::EventMask::PROPERTY_CHANGE
                    | x::EventMask::SUBSTRUCTURE_NOTIFY
                    | x::EventMask::SUBSTRUCTURE_REDIRECT,
            )],
        })
        .map_err(|_| Error::AlreadyRunning)?;

        let painter = Painter::new(conn, root, screen.default_colormap())?;
        let tags = Tags::new();
        let keys = Keys::new(conn, root)?;

        let display = Display::new(conn, root)?;

        conn.raw.send_and_check_request(&randr::SelectInput {
            window: root,
            enable: randr::NotifyMask::SCREEN_CHANGE
                | randr::NotifyMask::OUTPUT_CHANGE
                | randr::NotifyMask::CRTC_CHANGE
                | randr::NotifyMask::OUTPUT_PROPERTY,
        })?;

        let wm = WindowManager {
            signal: Arc::new(AtomicUsize::new(0)),
            display: display,
            painter: painter,
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
    pub fn next(&mut self, conn: &mut Connection<T>) -> Result<Event<T>, Error> {
        match conn.pop() {
            Some(e) => return Ok(e),
            None => {}
        }

        let event = conn.raw.wait_for_event()?;

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

        let e = match event {
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                let focus = self.display.get_focus();
                let value = self.keys.get(focus, e.state(), e.detail() as Keycode, true);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            }
            xcb::Event::X(xcb::x::Event::KeyRelease(ref e)) => {
                let focus = self.display.get_focus();
                let value = self
                    .keys
                    .get(focus, e.state(), e.detail() as Keycode, false);
                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            }
            xcb::Event::X(xcb::x::Event::ConfigureRequest(ref e)) => self.configure(conn, e),
            xcb::Event::X(xcb::x::Event::MapRequest(ref e)) => self.map(conn, e),
            xcb::Event::X(xcb::x::Event::EnterNotify(ref e)) => self.enter(conn, e),
            xcb::Event::X(xcb::x::Event::UnmapNotify(ref e)) => self.unmap(e),
            xcb::Event::X(xcb::x::Event::DestroyNotify(ref e)) => self.destroy(e),
            xcb::Event::RandR(xcb::randr::Event::ScreenChangeNotify(_)) => {
                self.display.update(conn)?;
                Ok(conn.pop().unwrap_or(Event::Empty))
            }
            xcb::Event::X(xcb::x::Event::ConfigureNotify(ref e)) => {
                if conn.root == e.window() {
                    /* this forces randr to update, flushing out any ScreenChangeNotifys */
                    self.display.reconfigure(conn, e.window())?;
                }

                Ok(Event::Empty)
            }
            _ => Ok(Event::Empty),
        };

        e
    }

    #[inline]
    pub fn bind(&mut self, conn: &mut Connection<T>, binding: &Binding<T>) -> Result<(), Error> {
        self.keys.bind(conn, binding)
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

    #[inline]
    pub fn get_monitor(&mut self) -> Option<MonitorId> {
        self.display.get_focus()
    }

    pub fn get_focus(&mut self) -> Option<(MonitorId, usize)> {
        self.get_monitor()
          .and_then(|focus| match self.get(focus) {
              Some(mon) => Some((focus, mon.focus)),
              None => None,
          })
    }
}

impl<T: Copy> WindowManager<T> {
    fn manage(&mut self, conn: &mut Connection<T>, window: x::Window) -> Result<Event<T>, Error> {
        let rect = Rect::new(0, 0, 0, 0);
        let client = Client::new(window, rect);
        let id = self.display.add(client);

        conn.send_and_check_request(&x::ChangeWindowAttributes {
            window: window,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::ENTER_WINDOW,
            )],
        })?;

        let mid = self.display.get_focus().unwrap();

        /* TODO: support empty output */
        Ok(Event::ClientCreate(mid, id))
    }

    fn enter(&mut self, conn: &mut Connection<T>, e: &x::EnterNotifyEvent) -> Result<Event<T>, Error> {
        let window = e.event();

        let focus = self.display.iter()
            .find_map(|(mid, mon)| {
                mon.find(window).map(|cid| {
                    (mid, cid)
                })
            });

        if let Some((mid, cid)) = focus {
            self.display.set_focus(conn, mid, cid)?;
        }

        Ok(Event::Empty)
    }

    /// handle a ConfigureRequestEvent, which is a request to configure a window's properties
    fn configure(&mut self, conn: &mut Connection<T>, event: &x::ConfigureRequestEvent) -> Result<Event<T>, Error> {
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

        conn.raw.send_and_check_request(&x::ConfigureWindow {
                window: event.window(),
                value_list: values.as_slice(),
            })?;

        Ok(Event::Empty)
    }

    /// handle the MapRequestEvent, which is a request for us to show a window on screen
    fn map(&mut self, conn: &mut Connection<T>, e: &x::MapRequestEvent) -> Result<Event<T>, Error> {
        let client = self
            .display
            .iter()
            .find_map(|(_, view)| view.find(e.window()));

        if client.is_none() {
            self.manage(conn, e.window())
        } else {
            Ok(Event::Empty)
        }
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
