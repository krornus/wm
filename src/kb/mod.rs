use std::collections::HashMap;

use crate::error::Error;

use xcb::x::{self, Keysym, Keycode};

pub mod keysym;

pub struct KeyMap {
    min: u32,
    max: u32,
    keymap: x::GetKeyboardMappingReply,
    modmap: x::GetModifierMappingReply,
}

pub struct KeycodeIterator<'a> {
    min: usize,
    per: usize,
    index: usize,
    target: Keysym,
    keysyms: &'a [Keysym],
}

impl<'a> Iterator for KeycodeIterator<'a> {
    type Item = Keycode;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.keysyms.len() {
            let i = self.index;

            /* next keysym in keycode */
            self.index += 1;

            if &self.keysyms[i] == &self.target {
                /* seek to next keycode */
                self.index = match self.index % self.per {
                    0 => self.index,
                    r => self.index + (self.per - r)
                };

                /* return keycode */
                return Some(((i / self.per) + self.min) as Keycode);
            }
        }

        None
    }
}

impl KeyMap {
    pub fn new(conn: &xcb::Connection) -> Result<Self, Error> {
        let setup = conn.get_setup();
        let min = setup.min_keycode();
        let max = setup.max_keycode();

        let key_cookie = conn.send_request(&x::GetKeyboardMapping {
            first_keycode: min,
            count: max - min + 1,
        });

        let mod_cookie = conn.send_request(&x::GetModifierMapping {});

        let keymap = conn.wait_for_reply(key_cookie)?;
        let modmap = conn.wait_for_reply(mod_cookie)?;

        Ok(KeyMap {
            min: min.into(),
            max: max.into(),
            keymap: keymap,
            modmap: modmap,
        })
    }

    pub fn keysyms(&self, keycode: Keycode) -> &[Keysym] {
        if (keycode as u32) < self.min || (keycode as u32) > self.max {
            &[]
        } else {
            let per = self.keymap.keysyms_per_keycode() as u32;
            let keysyms = self.keymap.keysyms();

            let start = (keycode as u32 - self.min) * per;
            let stop = start + per;

            &keysyms[start as usize..stop as usize]
        }
    }

    pub fn keycodes(&self, keysym: Keysym) -> KeycodeIterator {
        let per = self.keymap.keysyms_per_keycode() as usize;
        let keysyms = self.keymap.keysyms();

        KeycodeIterator {
            min: self.min as usize,
            per: per,
            index: 0,
            target: keysym,
            keysyms: keysyms,
        }
    }

    pub fn mask(&mut self, keysym: Keysym) -> Result<x::KeyButMask, Error> {
        for target in self.keycodes(keysym) {
            for (i, keycode) in self.modmap.keycodes().iter().enumerate() {
                if target == *keycode {
                    /* reply.keycodes really returns a 2 dimensional array,
                     *   keycodes[8][keycodes_per_modifier]
                     * by dividing the index by 8, we get the associated
                     * modifier, shifting it gives us the mask. */
                    let m =
                        x::KeyButMask::from_bits(1 << (i / 8)).unwrap_or(x::KeyButMask::empty());

                    return Ok(m);
                }
            }
        }

        Ok(x::KeyButMask::empty())
    }
}

pub struct KeyManager<T> {
    keymap: KeyMap,
    num_lock: x::KeyButMask,
    caps_lock: x::KeyButMask,
    scroll_lock: x::KeyButMask,
    bindings: HashMap<(x::KeyButMask, Keycode), T>,
}

impl<T: Copy> KeyManager<T> {
    /* TODO: support refreshing mappings */
    pub fn new(conn: &xcb::Connection) -> Result<Self, Error> {
        let mut keymap = KeyMap::new(conn)?;

        let num_lock = keymap.mask(keysym::Num_Lock)?;
        let caps_lock = keymap.mask(keysym::Caps_Lock)?;
        let scroll_lock = keymap.mask(keysym::Scroll_Lock)?;

        Ok(KeyManager {
            keymap: keymap,
            num_lock: num_lock,
            caps_lock: caps_lock,
            scroll_lock: scroll_lock,
            bindings: HashMap::new(),
        })
    }

    #[inline]
    fn grab(&self, conn: &xcb::Connection, root: x::Window, modifiers: x::KeyButMask, keycode: Keycode) -> xcb::VoidCookieChecked {
        let m = x::ModMask::from_bits(modifiers.bits()).unwrap();

        conn.send_request_checked(&x::GrabKey {
            owner_events: true,
            grab_window: root,
            modifiers: m,
            key: keycode as u8,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        })
    }

    pub fn bind(
        &mut self,
        conn: &xcb::Connection,
        root: x::Window,
        m: x::KeyButMask,
        k: Keysym,
        v: T,
    ) -> Result<(), Error> {
        let mut cookies = Vec::with_capacity(8);

        for kc in self.keymap.keycodes(k) {
            self.bindings.insert((m, kc), v);
            cookies.push(self.grab(
                conn, root, m, kc));
            cookies.push(self.grab(
                conn, root, m | self.num_lock, kc));
            cookies.push(self.grab(
                conn, root, m | self.caps_lock, kc));
            cookies.push(self.grab(
                conn, root, m | self.scroll_lock, kc));
            cookies.push(self.grab(
                conn, root, m | self.caps_lock   | self.num_lock, kc));
            cookies.push(self.grab(
                conn, root, m | self.scroll_lock | self.num_lock, kc));
            cookies.push(self.grab(
                conn, root, m | self.scroll_lock | self.caps_lock, kc));
            cookies.push(self.grab(
                conn, root, m | self.num_lock    | self.scroll_lock | self.caps_lock, kc));
        }

        for cookie in cookies {
            conn.check_request(cookie)?;
        }

        Ok(())
    }

    pub fn get(&self, m: x::KeyButMask, k: Keycode) -> Option<T> {
        self.bindings.get(&(m, k)).copied()
    }
}
