use std::collections::HashMap;

use crate::error::Error;
use crate::ffi::keyboard::*;

use xcb::x;

pub mod keysym;

pub type Keycode = xcb_keycode_t;
pub type Keysym = xcb_keysym_t;

pub struct KeySymbols<'a> {
    conn: &'a xcb::Connection,
    raw: *mut xcb_key_symbols_t,
}

pub struct KeycodeIterator {
    index: usize,
    keycodes: *mut xcb_keycode_t,
}

impl Drop for KeycodeIterator {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.keycodes as *mut libc::c_void);
        }
    }
}

impl Iterator for KeycodeIterator {
    type Item = Keycode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.keycodes.is_null() {
            return None;
        }

        let keycode = unsafe {
            self.keycodes.add(self.index)
                .as_ref()
                .unwrap()
        };

        if keycode != &x::NO_SYMBOL {
            self.index = self.index + 1;
            Some(*keycode)
        } else {
            None
        }
    }
}

impl<'a> KeySymbols<'a> {
    pub fn new(conn: &'a xcb::Connection) -> Result<Self, Error> {
        let ptr = conn.get_raw_conn();
        let raw = unsafe {
            xcb_key_symbols_alloc(ptr)
        };

        if raw.is_null() {
            Err(Error::KeyboardError)
        } else {
            Ok(KeySymbols {
                conn: conn,
                raw: raw,
            })
        }
    }

    pub fn keycodes(&mut self, keysym: Keysym) -> KeycodeIterator {
        let keycodes = unsafe {
            xcb_key_symbols_get_keycode(self.raw, keysym)
        };

        KeycodeIterator {
            index: 0,
            keycodes: keycodes,
        }
    }

    pub fn mask(&mut self, keysym: Keysym) -> Result<x::ModMask, Error> {
        let cookie = self.conn.send_request(&x::GetModifierMapping { });
        let reply = self.conn.wait_for_reply(cookie)?;

        for target in self.keycodes(keysym) {
            for (i, keycode) in reply.keycodes().iter().enumerate() {
                if target == (*keycode as u32) {
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

impl<'a> Drop for KeySymbols<'a> {
    fn drop(&mut self) {
        unsafe {
            xcb_key_symbols_free(self.raw);
        }
    }
}

pub struct KeyBinder<'a, T> {
    conn: &'a xcb::Connection,
    root: x::Window,
    sym: KeySymbols<'a>,
    map: Vec<(x::ModMask, Keycode, T)>,
}

impl<'a, T: Copy> KeyBinder<'a, T> {
    pub fn new(conn: &'a xcb::Connection, root: x::Window) -> Result<Self, Error> {
        let sym = KeySymbols::new(conn)?;
        let map = Vec::new();

        Ok(Self { conn, root, sym, map })
    }

    pub fn add(&mut self, m: x::KeyButMask, k: Keysym, v: T) {
        let modifiers = x::ModMask::from_bits(m.bits())
            .unwrap();

        for keycode in self.sym.keycodes(k) {
            self.map.push((modifiers, keycode, v));
        }
    }

    #[inline]
    fn grab(&self, modifiers: x::ModMask, keycode: u8) -> xcb::VoidCookieChecked {
        self.conn.send_request_checked(&x::GrabKey {
            owner_events: true,
            grab_window: self.root,
            modifiers: modifiers,
            key: keycode as u8,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        })
    }

    pub fn bind(mut self) -> Result<KeyManager<T>, Error> {
        let mut cookies = Vec::with_capacity(self.map.len() * 4 + 1);

        let numlock = self.sym.mask(keysym::Num_Lock)?;
        let capslock = self.sym.mask(keysym::Caps_Lock)?;

        let mut mgr = KeyManager::new();
        let map = std::mem::replace(&mut self.map, vec![]);

        let cookie = self.conn.send_request_checked(&x::UngrabKey {
            key: x::GRAB_ANY,
            grab_window: self.root,
            modifiers: x::ModMask::ANY,
        });

        cookies.push(cookie);

        for (m, k, v) in map {
            cookies.push(self.grab(m, k as u8));
            cookies.push(self.grab(m | numlock, k as u8));
            cookies.push(self.grab(m | capslock, k as u8));
            cookies.push(self.grab(m | numlock | capslock, k as u8));

            mgr.add(m, k, v);
        }

        for cookie in cookies {
            self.conn.check_request(cookie)?;
        }

        Ok(mgr)
    }
}

pub struct KeyManager<T> {
    map: HashMap<(x::ModMask, Keycode), T>,
}

impl<T: Copy> KeyManager<T> {
    pub fn new() -> Self {
        KeyManager {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, m: x::ModMask, k: Keycode, v: T) {
        self.map.insert((m, k), v);
    }

    pub fn get(&self, m: x::KeyButMask, k: Keycode) -> Option<T> {
        let modifiers = x::ModMask::from_bits(m.bits())
            .unwrap();

        self.map.get(&(modifiers, k)).copied()
    }
}
