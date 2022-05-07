use std::collections::HashMap;

use crate::error::Error;
use crate::wm::Adapter;

use xcb::x::{self, Keysym, Keycode};

pub mod keysym;

pub struct KeyMap {
    min: u32,
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

        let mod_cookie = conn.send_request(&x::GetModifierMapping {
        });

        let keymap = conn.wait_for_reply(key_cookie)?;
        let modmap = conn.wait_for_reply(mod_cookie)?;

        Ok(KeyMap {
            min: min.into(),
            keymap: keymap,
            modmap: modmap,
        })
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
    root: x::Window,
    keymap: KeyMap,
    num_lock: x::KeyButMask,
    caps_lock: x::KeyButMask,
    scroll_lock: x::KeyButMask,
    bindings: HashMap<(x::KeyButMask, Keycode), T>,
}

impl<T: Copy> KeyManager<T> {
    /* TODO: support refreshing mappings */
    pub fn new(conn: &xcb::Connection) -> Result<Self, Error> {
        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .next()
            .ok_or(Error::MissingScreen)?;

        let root = screen.root();

        let mut keymap = KeyMap::new(conn)?;

        let num_lock = keymap.mask(keysym::Num_Lock)?;
        let caps_lock = keymap.mask(keysym::Caps_Lock)?;
        let scroll_lock = keymap.mask(keysym::Scroll_Lock)?;

        Ok(KeyManager {
            root: root,
            keymap: keymap,
            num_lock: num_lock,
            caps_lock: caps_lock,
            scroll_lock: scroll_lock,
            bindings: HashMap::new(),
        })
    }

    #[inline]
    fn grab(&self, adapter: &mut Adapter, modifiers: x::KeyButMask, keycode: Keycode) {
        let m = x::ModMask::from_bits(modifiers.bits()).unwrap();

        println!("grab: [{:?} + {:?}]", modifiers, keycode);

        adapter.request(&x::GrabKey {
            owner_events: true,
            grab_window: self.root,
            modifiers: m,
            key: keycode as u8,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        })
    }

    pub fn bind(
        &mut self,
        adapter: &mut Adapter,
        m: x::KeyButMask,
        k: Keysym,
        v: T,
    ) -> Result<(), Error> {
        for kc in self.keymap.keycodes(k) {
            self.bindings.insert((m, kc), v);

            self.grab(adapter, m, kc);
            self.grab(adapter, m | self.num_lock, kc);
            self.grab(adapter, m | self.caps_lock, kc);
            self.grab(adapter, m | self.scroll_lock, kc);
            self.grab(adapter, m | self.caps_lock | self.num_lock, kc);
            self.grab(adapter, m | self.scroll_lock | self.num_lock, kc);
            self.grab(adapter, m | self.scroll_lock | self.caps_lock, kc);
            self.grab(adapter, m | self.num_lock | self.scroll_lock | self.caps_lock, kc);
        }

        adapter.check()?;

        Ok(())
    }

    pub fn get(&self, mut m: x::KeyButMask, k: Keycode) -> Option<T> {
        println!("get: [{:?} + {:?}]", m, k);
        m.remove(self.num_lock | self.caps_lock | self.scroll_lock);
        println!("  -> [{:?} + {:?}]", m, k);
        self.bindings.get(&(m, k)).copied()
    }
}
