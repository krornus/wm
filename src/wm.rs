use std::ptr;
use std::sync::Arc;
use std::process::Command;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::os::unix::prelude::{AsRawFd, RawFd};

use fork::{Fork};
use xcb::{self, x};
use signal_hook::consts::signal::*;

use crate::rect::Rect;
use crate::error::Error;
use crate::kb::{KeySymbols, Keysym, Keycode, keysym};
use crate::layout::{Layout, LeftMaster};

pub struct Monitor {
    area: Rect,
}

impl Monitor {
    fn new(screen: &x::Screen) -> Self {
        Monitor {
            area: Rect::new(
                0, 0,
                screen.width_in_pixels() as usize,
                screen.height_in_pixels() as usize,
            )
        }
    }
}

pub struct WindowManager<T> {
    root: x::Window,
    conn: xcb::Connection,
    sym: KeySymbols,
    pending: Vec<xcb::VoidCookieChecked>,
    signal: Arc<AtomicUsize>,
    keymap: KeyManager<T>,
    layout: LeftMaster,
    monitors: Vec<Monitor>,
    numlock: x::ModMask,
    capslock: x::ModMask,
}

pub enum Event<T> {
    Empty,
    Interrupt,
    UserEvent(T),
    Map(x::Window),
}

impl<T: Copy> WindowManager<T> {
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect(name)?;

        let setup = conn.get_setup();
        let scr = setup
            .roots()
            .nth(main as usize)
            .ok_or(Error::MissingScreen)?;

        let monitors = setup
            .roots()
            .map(|s| Monitor::new(s))
            .collect();

        let root = scr.root();
        let sym = KeySymbols::new(&conn)?;

        let cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::SUBSTRUCTURE_REDIRECT |
                x::EventMask::KEY_PRESS,
            )],
        });

        conn.check_request(cookie)
            .map_err(|_| Error::AlreadyRunning)?;

        let mut wm = WindowManager {
            conn: conn,
            root: root,
            signal: Arc::new(AtomicUsize::new(0)),
            sym: sym,
            keymap: KeyManager::new(),
            pending: Vec::new(),
            monitors: monitors,
            layout: LeftMaster::new(),
            numlock: x::ModMask::empty(),
            capslock: x::ModMask::empty(),
        };

        wm.numlock = wm.modmask(keysym::Num_Lock)?;
        wm.capslock = wm.modmask(keysym::Caps_Lock)?;

        println!("numlock: {:#x}", wm.numlock.bits());
        println!("capslock: {:#x}", wm.capslock.bits());

        signal_hook::flag::register_usize(SIGCHLD, Arc::clone(&wm.signal), SIGCHLD as usize)
            .map_err(|e| Error::SignalError(e))?;

        signal_hook::flag::register_usize(SIGINT, Arc::clone(&wm.signal), SIGINT as usize)
            .map_err(|e| Error::SignalError(e))?;

        Self::reap()?;

        Ok(wm)
    }

    pub fn next(&mut self) -> Result<Event<T>, Error> {
        self.sync()?;

        let event = self.conn.wait_for_event()?;

        const SIGCHLD_U: usize = SIGCHLD as usize;
        const SIGINT_U:  usize = SIGINT as usize;

        match self.signal.load(Ordering::Relaxed) {
            SIGCHLD_U => { Self::reap()?; },
            SIGINT_U => { return Ok(Event::Interrupt); },
            _ => { },
        }

        match event {
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                let keysym = self.sym.keysym(e.detail() as Keycode, 0);
                let value = self.keymap.get(e.state(), keysym);

                Ok(value.map_or(Event::Empty, |x| Event::UserEvent(x)))
            },
            xcb::Event::X(xcb::x::Event::MapRequest(ref e)) => {
                Ok(Event::Map(e.window()))
            },
            xcb::Event::X(xcb::x::Event::ConfigureRequest(ref e)) => {
                self.configure(e);
                Ok(Event::Empty)
            },
            _ => Ok(Event::Empty),
        }
    }

    pub fn bind(&mut self, m: x::KeyButMask, k: Keysym, v: T) {
        self.keymap.add(m, k, v);

        let modifiers = x::ModMask::from_bits(m.bits())
            .unwrap();

        for keycode in self.sym.keycodes(k) {
            self.grabkey(modifiers, keycode as u8);
            self.grabkey(modifiers | self.numlock, keycode as u8);
            self.grabkey(modifiers | self.capslock, keycode as u8);
            self.grabkey(modifiers | self.numlock | self.capslock, keycode as u8);
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

    pub fn map(&mut self, win: x::Window) {
        self.request(&x::ChangeWindowAttributes {
            window: win,
            value_list: &[xcb::x::Cw::EventMask(
                x::EventMask::SUBSTRUCTURE_REDIRECT
            )],
        });

        self.request(&x::MapWindow {
            window: win,
        });
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
            let rv = unsafe {
                libc::waitpid(
                    -1,
                    ptr::null::<*const i32>() as *mut i32,
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

    fn grabkey(&mut self, modifiers: x::ModMask, keycode: u8) {
        self.request(&x::GrabKey {
            owner_events: true,
            grab_window: self.root,
            modifiers: modifiers,
            key: keycode as u8,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        });
    }

    fn modmask(&mut self, keysym: Keysym) -> Result<x::ModMask, Error> {
        let cookie = self.conn.send_request(&x::GetModifierMapping { });
        let reply = self.conn.wait_for_reply(cookie)?;

        for target in self.sym.keycodes(keysym) {
            for (i, keycode) in reply.keycodes().iter().enumerate() {
                if target == (*keycode as u32) {
                    println!("foudn it");
                    /* reply.keycodes really returns a 2 dimensional array,
                     *   keycodes[8][keycodes_per_modifier]
                     * by dividing the index by 8, we get the associated
                     * modifier, shifting it gives us the mask. */
                    let m = x::ModMask::from_bits(1 << (i / 8))
                        .unwrap_or(x::ModMask::empty());

                    return Ok(m);
                }
            }
        }

        Ok(x::ModMask::empty())
    }
}

impl<T> AsRawFd for WindowManager<T> {
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
