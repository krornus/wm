use crate::error::Error;
use crate::ffi::keyboard::*;

pub mod keysym;

pub type Keycode = xcb_keycode_t;
pub type Keysym = xcb_keysym_t;

pub struct KeySymbols {
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

        if keycode != &xcb::x::NO_SYMBOL {
            self.index = self.index + 1;
            Some(*keycode)
        } else {
            None
        }
    }
}

impl KeySymbols {
    pub fn new(conn: &xcb::Connection) -> Result<Self, Error> {
        let ptr = conn.get_raw_conn();
        let raw = unsafe {
            xcb_key_symbols_alloc(ptr)
        };

        if raw.is_null() {
            Err(Error::KeyboardError)
        } else {
            Ok(KeySymbols {
                raw: raw,
            })
        }
    }

    pub fn keysym(&mut self, code: Keycode, col: i32) -> Keysym {
        unsafe {
            xcb_key_symbols_get_keysym(self.raw, code, col)
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
}

impl Drop for KeySymbols {
    fn drop(&mut self) {
        unsafe {
            xcb_key_symbols_free(self.raw);
        }
    }
}
