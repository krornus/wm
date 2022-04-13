use std::ptr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::collections::HashMap;
use std::os::unix::prelude::{AsRawFd, RawFd};

use thiserror::Error;
use signal_hook::consts::signal::*;
use xcb::{self, x};
use xkbcommon::xkb;
use libc;

#[derive(Error, Debug)]
pub enum Error {
    #[error("a window manager is already running")]
    AlreadyRunning,
    #[error("screen not found")]
    MissingScreen,
    #[error("XKB version unsupported")]
    XKBUnsupported,
    #[error("Unknown keyboard device")]
    UnknownKeyboard,
    #[error("failed to register signal handler")]
    SignalError(std::io::Error),
    #[error("failed to connect to X11 server")]
    ConnectionError(#[from] xcb::ConnError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("xcb error")]
    XCBError(#[from] xcb::Error),
    #[error("protocol error")]
    ProtocolError(#[from] xcb::ProtocolError),
}

pub struct Key {
    state: x::KeyButMask,
    key: x::Keysym,
}

struct Keyboard {
    context: xkb::Context,
    keymap: xkb::Keymap,
    state: xkb::State,
}

impl Keyboard {
    pub fn new(conn: &xcb::Connection) -> Result<Self, Error> {
        Self::from_id(conn, Self::core_id(conn)?)
    }

    pub fn from_id(conn: &xcb::Connection, id: i32) -> Result<Self, Error> {
        Self::select(conn)?;

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb::x11::keymap_new_from_device(
            &context,
            conn,
            id,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );

        let state = xkb::x11::state_new_from_device(
            &keymap,
            conn,
            id);

        Ok(Keyboard {
            context: context,
            keymap: keymap,
            state: state,
        })
    }

    fn core_id(conn: &xcb::Connection) -> Result<i32, Error> {
        let cookie = conn.send_request(&xcb::xkb::UseExtension {
            wanted_major: xkb::x11::MIN_MAJOR_XKB_VERSION,
            wanted_minor: xkb::x11::MIN_MINOR_XKB_VERSION,
        });

        let version = conn.wait_for_reply(cookie)?;
        if !version.supported() {
            return Err(Error::XKBUnsupported);
        }

        let id = xkb::x11::get_core_keyboard_device_id(conn);
        if id < 0 {
            Err(Error::UnknownKeyboard)
        } else {
            Ok(id)
        }
    }

    fn select(conn: &xcb::Connection) -> Result<(), Error> {
        /* c equivalent -- xcb_xkb_select_events */
        let events =
            xcb::xkb::EventType::NEW_KEYBOARD_NOTIFY |
            xcb::xkb::EventType::MAP_NOTIFY |
            xcb::xkb::EventType::STATE_NOTIFY;

        let map_parts =
            xcb::xkb::MapPart::KEY_TYPES |
            xcb::xkb::MapPart::KEY_SYMS |
            xcb::xkb::MapPart::MODIFIER_MAP |
            xcb::xkb::MapPart::EXPLICIT_COMPONENTS |
            xcb::xkb::MapPart::KEY_ACTIONS |
            xcb::xkb::MapPart::KEY_BEHAVIORS |
            xcb::xkb::MapPart::VIRTUAL_MODS |
            xcb::xkb::MapPart::VIRTUAL_MOD_MAP;

        let spec = unsafe {
            std::mem::transmute::<_, u32>(xcb::xkb::Id::UseCoreKbd)
        };

        let cookie = conn.send_request_checked(&xcb::xkb::SelectEvents {
            device_spec: spec as xcb::xkb::DeviceSpec,
            affect_which: events,
            clear: xcb::xkb::EventType::empty(),
            select_all: events,
            affect_map: map_parts,
            map: map_parts,
            details: &[],
        });

        conn.check_request(cookie)?;

        Ok(())
    }

    pub fn update_mask(&mut self, ev: &xcb::xkb::StateNotifyEvent) {
        self.state.update_mask(
            ev.base_mods().bits() as xkb::ModMask,
            ev.latched_mods().bits() as xkb::ModMask,
            ev.locked_mods().bits() as xkb::ModMask,
            ev.base_group() as xkb::LayoutIndex,
            ev.latched_group() as xkb::LayoutIndex,
            ev.locked_group() as xkb::LayoutIndex,
        );
    }

    pub fn keysym(&self, ev: &xcb::x::KeyPressEvent) -> xcb::x::Keysym {
        println!("keypress: {}", self.state.key_get_utf8(ev.detail() as u32));
        self.state.key_get_one_sym(ev.detail() as u32) as xcb::x::Keysym
    }
}


#[allow(non_snake_case)]
pub struct Atoms {
    WMProtocols: x::Atom,
    WMDelete: x::Atom,
    WMState: x::Atom,
    NetSupported: x::Atom,
    NetWMName: x::Atom,
    NetWMState: x::Atom,
    NetWMFullscreen: x::Atom,
}

impl Atoms {
    fn request(conn: &xcb::Connection, name: &[u8]) -> x::InternAtomCookie {
        conn.send_request(&x::InternAtom {
            only_if_exists: false,
            name: name,
        })
    }

    fn load(conn: &xcb::Connection) -> Result<Self, Error> {
        let prot =  Self::request(conn, b"WM_PROTOCOLS");
        let del = Self::request(conn, b"WM_DELETE_WINDOW");
        let state = Self::request(conn, b"WM_STATE");
        let supp = Self::request(conn, b"_NET_SUPPORTED");
        let name = Self::request(conn, b"_NET_WM_NAME");
        let netstate = Self::request(conn, b"_NET_WM_STATE");
        let fullscreen = Self::request(conn, b"_NET_WM_STATE_FULLSCREEN");

        Ok(Atoms {
            WMProtocols: conn.wait_for_reply(prot)?.atom(),
            WMDelete: conn.wait_for_reply(del)?.atom(),
            WMState: conn.wait_for_reply(state)?.atom(),
            NetSupported: conn.wait_for_reply(supp)?.atom(),
            NetWMName: conn.wait_for_reply(name)?.atom(),
            NetWMState: conn.wait_for_reply(netstate)?.atom(),
            NetWMFullscreen: conn.wait_for_reply(fullscreen)?.atom(),
        })
    }
}

pub struct WindowManager {
    conn: xcb::Connection,
    root: x::Window,
    sigchld: Arc<AtomicUsize>,
    sigint: Arc<AtomicUsize>,
    atoms: Atoms,
    keyboard: Keyboard,
}

impl WindowManager {
    pub fn connect(name: Option<&str>) -> Result<Self, Error> {
        let (conn, main) = xcb::Connection::connect(name)?;

        let setup = conn.get_setup();
        let scr = setup.roots()
             .nth(main as usize)
             .ok_or(Error::MissingScreen)?;

        let root = scr.root();
        let atoms = Atoms::load(&conn)?;

        let keyboard = Keyboard::new(&conn)?;

        let mut wm = WindowManager {
            conn: conn,
            root: root,
            sigchld: Arc::new(AtomicUsize::new(0)),
            sigint: Arc::new(AtomicUsize::new(0)),
            atoms: atoms,
            keyboard: keyboard,
            keys: HashMap::new(),
        };

        signal_hook::flag::register_usize(
            SIGCHLD,
            Arc::clone(&wm.sigchld),
            SIGCHLD as usize
        ).map_err(|e| Error::SignalError(e))?;

        signal_hook::flag::register_usize(
            SIGINT,
            Arc::clone(&wm.sigint),
            SIGINT as usize
        ).map_err(|e| Error::SignalError(e))?;

        wm.lock()?;
        wm.select()?;

        Self::reap()?;

        Ok(wm)
    }

    pub fn handle(&mut self) -> Result<(), Error> {
        let event = self.conn.wait_for_event()?;

        match event {
            xcb::Event::Xkb(xcb::xkb::Event::StateNotify(ref e)) => {
                self.keyboard.update_mask(e);
            },
            xcb::Event::X(xcb::x::Event::KeyPress(ref e)) => {
                self.keyboard.keysym(e);
            },
            _ => { }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            self.handle()?;
        }
    }

    /* cleanup all dead child processes */
    fn reap() -> Result<bool, Error> {
        let mut zombie = false;

        loop {
            let rv = unsafe {
                libc::waitpid(
                    -1, ptr::null::<*const i32>() as *mut i32, libc::WNOHANG)
            };

            if rv < 0 {
                let e = std::io::Error::last_os_error();
                let errno = std::io::Error::raw_os_error(&e);
                match errno {
                    Some(libc::ECHILD) => break Ok(zombie),
                    Some(x) => break Err(Error::IoError(e)),
                    None => unreachable!(),
                }
            } else if rv == 0 {
                break Ok(zombie)
            } else {
                zombie = true;
            }
        }
    }

    fn lock(&mut self) -> Result<(), Error> {
        /* The window manager enforces its window layout policy using
         * substructure redirection. When the window manager selects
         * SubstructureRedirectMask on the root window, an attempt by any other
         * client to change the configuration of any child of the root window
         * will fail. - O'Reilly */
        let cookie = self.conn.send_request_checked(&x::ChangeWindowAttributes {
            window: self.root,
            value_list: &[
                xcb::x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_REDIRECT)
            ],
        });

        self.conn.check_request(cookie)
            .map_err(|_| Error::AlreadyRunning)
    }

    fn select(&mut self) -> Result<(), Error> {
        let cookie = self.conn.send_request_checked(&x::ChangeWindowAttributes {
            window: self.root,
            value_list: &[
                xcb::x::Cw::EventMask(
                    x::EventMask::SUBSTRUCTURE_REDIRECT |
                    x::EventMask::KEY_PRESS
                )
            ],
        });

        self.conn.check_request(cookie)?;
        Ok(())
    }
}


impl AsRawFd for WindowManager {
    fn as_raw_fd(&self) -> RawFd {
        self.conn.as_raw_fd()
    }
}
