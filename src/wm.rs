use std::sync::Arc;
use std::os::unix::prelude::{AsRawFd, RawFd};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

use fork::Fork;
use signal_hook::consts::signal::*;
use xcb::x::{self, Keysym, Keycode};

use crate::error::Error;
use crate::kb::KeyManager;

const DEFAULT_TAGS: u64 = 0x1;

#[derive(Clone, PartialEq, Eq)]
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
    fn arrange(&mut self, scope: &Rect, count: usize, index: usize, focus: bool) -> Option<Rect>;
}

pub struct LeftMaster { }

impl Layout for LeftMaster {
    fn arrange(&mut self, scope: &Rect, count: usize, index: usize, _: bool) -> Option<Rect> {
        let rect = if index == 0 {
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
        };

        Some(rect)
    }
}

pub struct RightMaster { }

impl Layout for RightMaster {
    fn arrange(&mut self, scope: &Rect, count: usize, index: usize, _: bool) -> Option<Rect> {
        let rect = if index == 0 {
            if count == 1 {
                Rect::new(0, 0, scope.w, scope.h)
            } else {
                Rect::new(scope.center_x(), 0, scope.center_x(), scope.h)
            }
        } else {
            /* height of one box */
            let boxh = scope.h / (count - 1);
            /* pos of one box */
            let posh = boxh * (index - 1);

            Rect::new(0, posh, scope.center_x(), posh + boxh)
        };

        Some(rect)
    }
}

pub struct Monacle { }

impl Layout for Monacle {
    fn arrange(&mut self, scope: &Rect, _: usize, _: usize, focus: bool) -> Option<Rect> {
        if focus {
            Some(scope.clone())
        } else {
            None
        }
    }
}

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

pub struct Client {
    scope: Rect,
    window: x::Window,
    visible: bool,
}

impl Client {
    fn new(window: x::Window) -> Self {
        Client {
            scope: Rect::new(0, 0, 1, 1),
            window: window,
            visible: false,
        }
    }

    fn resize(&mut self, adapter: &mut Adapter, scope: Rect) {
        if self.scope != scope {
            self.scope = scope;

            adapter.request(&x::ConfigureWindow {
                window: self.window,
                value_list: &[
                    x::ConfigWindow::X(self.scope.x as i32),
                    x::ConfigWindow::Y(self.scope.y as i32),
                    x::ConfigWindow::Width(self.scope.w as u32),
                    x::ConfigWindow::Height(self.scope.h as u32),
                ],
            });
        }
    }

    fn show(&mut self, adapter: &mut Adapter, visible: bool) {
        if self.visible != visible {
            self.visible = visible;

            if visible {
                adapter.request(&x::MapWindow {
                    window: self.window,
                });
            } else {
                adapter.request(&x::UnmapWindow {
                    window: self.window,
                });
            }
        }
    }

    fn focus(&mut self, adapter: &mut Adapter) {
        if self.visible {
            adapter.request(&x::SetInputFocus {
                revert_to: x::InputFocus::PointerRoot,
                focus: self.window,
                time: x::CURRENT_TIME,
            });
        }
    }
}

pub struct Screen {
    scope: Rect,
    root: x::Window,
    clients: Vec<Client>,
    selclient: Option<usize>,
    layout: Box<dyn Layout>,
}

impl Screen {
    fn new<T: 'static + Layout>(scr: &x::Screen, layout: T) -> Self {
        let scope = Rect::new(
            0, 0,
            scr.width_in_pixels() as usize,
            scr.height_in_pixels() as usize,
        );

        Screen {
            scope: scope,
            root: scr.root(),
            clients: vec![],
            selclient: None,
            layout: Box::new(layout),
        }
    }

    pub fn focus(&self) -> Option<&Client> {
        self.selclient.map(|x| &self.clients[x])
    }

    pub fn focus_mut(&mut self) -> Option<&mut Client> {
        self.selclient.map(move |x| &mut self.clients[x])
    }

    fn client(&self, window: x::Window) -> Option<&Client> {
        self.clients.iter().find(|x| x.window == window)
    }

    fn client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.clients.iter_mut().find(|x| x.window == window)
    }

    fn arrange(&mut self, adapter: &mut Adapter) -> Result<(), Error> {
        let count = self.clients.len();

        for (i, client) in self.clients.iter_mut().enumerate() {
            let focus = match self.selclient {
                Some(x) => x == i,
                _ => false,
            };

            if let Some(scope) = self.layout.arrange(&self.scope, count, i, focus) {
                client.show(adapter, true);
                client.resize(adapter, scope);
            } else {
                client.show(adapter, false);
            }
        }

        adapter.check()?;

        Ok(())
    }

    fn add(&mut self, adapter: &mut Adapter, client: Client) -> Result<&mut Client, Error> {
        self.clients.push(client);
        let idx = self.clients.len() - 1;
        self.selclient = Some(idx);

        self.arrange(adapter)?;

        Ok(&mut self.clients[idx])
    }

    fn remove(&mut self, adapter: &mut Adapter, window: x::Window) -> Result<(), Error> {
        if let Some(pos) = self.clients.iter().position(|x| x.window == window) {
            self.clients.remove(pos);
            let idx = self.clients.len() - 1;
            self.selclient = Some(idx);
            self.arrange(adapter)
        } else {
            Ok(())
        }
    }

   fn set_focus(&mut self, adapter: &mut Adapter, selclient: Option<usize>) -> Result<(), Error> {

        if let Some(i) = self.selclient {
            self.clients[i].focus(adapter);
        } else {
            adapter.request(&x::SetInputFocus {
                revert_to: x::InputFocus::PointerRoot,
                focus: self.root,
                time: x::CURRENT_TIME,
            });
        };

        self.arrange(adapter)
    }
}


/* create a mask from a set of indices */
macro_rules! mask {
    ( $( $x:expr ),* ) => {
        $( (1 << $x) )|*
    };
}

pub type TagSetID = usize;

struct TagSet {
    tags: Vec<String>,
}

impl TagSet {
    fn new(tags: Vec<&str>) -> Self {
        if tags.len() > u64::BITS as usize {
            /* TODO: probably use a bitvec */
            panic!("too many tags (max: {})", u64::BITS);
        }

        let from = tags
            .into_iter()
            .map(String::from)
            .collect();

        TagSet {
            tags: from,
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.tags.len()
    }

    #[inline]
    fn index(&self, name: &str) -> Option<usize> {
        self.tags.iter().position(|x| x == name)
    }
}

pub struct TagSelection {
    map: HashMap<TagSetID, u64>,
}

pub enum Tag {
    On(u64),
    Off(u64),
    Toggle(u64),
    Value(u64),
}

impl TagSelection {
    pub fn new() -> Self {
        TagSelection {
            map: HashMap::new()
        }
    }

    pub fn update(&mut self, identifier: TagSetID, tag: Tag) -> u64 {
        let mut mask = self.map.get(&identifier).copied().unwrap_or(0);

        mask = match tag {
            Tag::On(m) => {
                mask | m
            },
            Tag::Off(m) => {
                mask & (!m)
            },
            Tag::Toggle(m) => {
                mask ^ m
            },
            Tag::Value(m) => {
                m
            },
        };

        self.map.insert(identifier, mask);
        mask
    }

    #[inline]
    pub fn get(&self, identifier: TagSetID) -> Option<u64> {
        self.map.get(&identifier).copied()
    }
}

pub struct TagManager {
    identifier: TagSetID,
    selection: TagSelection,
    tagsets: HashMap<TagSetID, TagSet>,
}

impl TagManager {
    pub fn new() -> Self {
        TagManager {
            identifier: 0,
            selection: TagSelection::new(),
            tagsets: HashMap::new(),
        }
    }

    #[inline]
    pub fn update(&mut self, identifier: TagSetID, action: Tag) -> u64 {
        self.selection.update(identifier, action)
    }

    pub fn add(&mut self, tags: Vec<&str>) -> TagSetID {
        assert!(tags.len() > 0);
        /* get id, update next id */
        let identifier = self.identifier;
        self.identifier = self.identifier + 1;
        /* add the tagset */
        self.tagsets.insert(identifier, TagSet::new(tags));
        /* enable the default tag */
        self.update(identifier, Tag::Value(mask!(0)));

        identifier
    }

    pub fn visible(&self, selection: &TagSelection) -> bool {
        for (identifier, mask) in selection.map.iter() {
            if let Some(other) = self.selection.get(*identifier) {
                if mask & other == 0 {
                    return false;
                }
            }
        }

        true
    }
}

pub struct WindowManager<T: Copy> {
    adapter: Adapter,
    keys: KeyManager<T>,
    tags: TagManager,
    selmon: usize,
    monitors: Vec<Screen>,
    signal: Arc<AtomicUsize>,
}

pub enum Event<T> {
    Empty,
    Interrupt,
    UserEvent(T),
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

        let keys = KeyManager::new(&conn)?;
        let monitors = setup.roots().map(|x| {
            Screen::new(x, LeftMaster {})
        }).collect();

        let wm = WindowManager {
            keys: keys,
            selmon: 0,
            monitors: monitors,
            tags: TagManager::new(),
            signal: Arc::new(AtomicUsize::new(0)),
            adapter: Adapter::new(conn),
        };

        signal_hook::flag::register_usize(SIGCHLD, Arc::clone(&wm.signal), SIGCHLD as usize)
            .map_err(|e| Error::SignalError(e))?;
        signal_hook::flag::register_usize(SIGINT, Arc::clone(&wm.signal), SIGINT as usize)
            .map_err(|e| Error::SignalError(e))?;

        Self::reap()?;

        Ok(wm)
    }

    /// Get the next event from the queue. this may be combined with
    /// polling via the AsRawFd trait of the WindowManager struct.
    /// Note that this function handles signal interrupts for SIGINT
    /// and SIGCHLD, so you should call next() if your polling is
    /// interrupted by a signal.
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
            xcb::Event::X(xcb::x::Event::EnterNotify(ref e)) => {
                self.enter(e)
            },
            xcb::Event::X(xcb::x::Event::ConfigureRequest(ref e)) => {
                self.configure(e)
            },
            _ => {
                Ok(Event::Empty)
            },
        }
    }

    pub fn tagset(&mut self, tagset: Vec<&str>) -> TagSetID {
        self.tags.add(tagset)
    }

    /// spawn a subprogram
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

    /// move focus to an optional client
    pub fn set_focus(&mut self, index: Option<isize>) -> Result<(), Error> {
        let selclient = if let Some(x) = index {
            let now = match self.monitors[self.selmon].selclient {
                Some(x) => x,
                None => 0,
            };

            let next = if x < 0 {
                now.saturating_sub(x.saturating_neg() as usize)
            } else {
                now.saturating_add(x as usize)
            };

            Some(next)
        } else {
            None
        };

        self.monitors[self.selmon].set_focus(&mut self.adapter, selclient)
    }

    pub fn focus(&self) -> Option<&Client> {
        None
    }

    pub fn focus_mut(&mut self) -> Option<&mut Client> {
        None
    }

    /// bind a (mask, key) combo to return a UserEvent when pressed
    pub fn bind(&mut self, m: x::KeyButMask, k: Keysym, v: T) -> Result<(), Error> {
        self.keys.bind(&mut self.adapter, m, k, v)
    }

    pub fn monitors(&self) -> &[Screen] {
        &self.monitors
    }

    pub fn monitors_mut(&mut self) -> &mut [Screen] {
        &mut self.monitors
    }
}

impl<T: Copy> WindowManager<T> {
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

        self.adapter.conn.send_and_check_request(&x::ConfigureWindow {
            window: event.window(),
            value_list: values.as_slice(),
        })?;

        Ok(Event::Empty)
    }

    /// handle the MapRequestEvent, which is a request for us to show a window on screen
    fn map(&mut self, e: &x::MapRequestEvent) -> Result<Event<T>, Error> {
        let c = match self.client_mut(e.window()) {
            Some(c) => c,
            None => {
                self.monitors[self.selmon].add(
                    &mut self.adapter,
                    Client::new(e.window()))?
            }
        };

        Ok(Event::Empty)
    }

    /// handle the UnmapNotifyEvent, which notifies us that a window has been unmapped (hidden)
    fn unmap(&mut self, e: &x::UnmapNotifyEvent) -> Result<Event<T>, Error> {
        self.client_mut(e.window())
            .map(|c| c.visible = false);

        Ok(Event::Empty)
    }

    /// handle the DestroyNotify, which notifies us that a window has been destroyed
    fn destroy(&mut self, e: &x::DestroyNotifyEvent) -> Result<Event<T>, Error> {
        self.monitors[self.selmon].remove(&mut self.adapter, e.window())?;
        Ok(Event::Empty)
    }

    /// handle the EnterNotifyEvent, which notifies us that focus has entered a given window
    fn enter(&mut self, e: &x::EnterNotifyEvent) -> Result<Event<T>, Error> {
        if let Some((i, j)) = self.client_index(e.event()) {
            self.selmon = i;
            self.monitors[self.selmon].selclient = Some(j);
        }

        Ok(Event::Empty)
    }

    /// get monitor, client indices based its window
    fn client_index(&mut self, window: x::Window) -> Option<(usize, usize)> {
        for (i, mon) in self.monitors.iter().enumerate() {
            for (j, client) in mon.clients.iter().enumerate() {
                if client.window == window {
                    return Some((i, j))
                }
            }
        }

        None
    }

    /// get a reference to a client based on its window
    fn client(&self, window: x::Window) -> Option<&Client> {
        self.monitors.iter()
            .find_map(|x| x.client(window))
    }

    /// get a mutable reference to a client based on its window
    fn client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.monitors.iter_mut()
            .find_map(|x| x.client_mut(window))
    }

    /// cleans up all completed child processes
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

impl<T: Copy> AsRawFd for WindowManager<T> {
    /* for use with epoll etc... */
    fn as_raw_fd(&self) -> RawFd {
        self.adapter.conn.as_raw_fd()
    }
}
