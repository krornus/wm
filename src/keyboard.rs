use std::collections::HashMap;

use crate::display::MonitorId;
use crate::error::Error;
use crate::keysym;
use crate::wm::Connection;

use bitflags::bitflags;
use xcb::x::{self, Keycode, Keysym};

bitflags! {
    pub struct Modifier: u32 {
        const SHIFT = 0x00000001;
        const LOCK = 0x00000002;
        const CONTROL = 0x00000004;
        const MOD1 = 0x00000008;
        const MOD2 = 0x00000010;
        const MOD3 = 0x00000020;
        const MOD4 = 0x00000040;
        const MOD5 = 0x00000080;
        const ANY = 0x00008000;
    }
}

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
                    r => self.index + (self.per - r),
                };

                /* return keycode */
                return Some(((i / self.per) + self.min) as Keycode);
            }
        }

        None
    }
}

impl KeyMap {
    pub fn new<T>(conn: &mut Connection<T>) -> Result<Self, Error> {
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

    pub fn mask(&mut self, keysym: Keysym) -> Result<Modifier, Error> {
        /* taken from i3 */
        let modmap = self.modmap.keycodes();
        let keycodes_per_modifier = modmap.len() / 8;

        for modifier in 0..8 {
            for j in 0..keycodes_per_modifier {
                let modcode = modmap[(modifier * keycodes_per_modifier) + j];

                for keycode in self.keycodes(keysym) {
                    if keycode == modcode {
                        return unsafe { Ok(Modifier::from_bits_unchecked(1 << modifier)) };
                    }
                }
            }
        }

        Ok(Modifier::empty())
    }
}

#[derive(Copy, Clone)]
pub enum Press {
    Press,
    Release,
    Both,
}

pub struct Binding<T: Copy> {
    pub view: Option<MonitorId>,
    pub mask: Modifier,
    pub keysym: Keysym,
    pub press: Press,
    pub value: T,
}

#[derive(Clone)]
struct BindValue<T: Copy> {
    view: Option<MonitorId>,
    value: T,
}

impl<T: Copy> From<&Binding<T>> for BindValue<T> {
    fn from(binding: &Binding<T>) -> Self {
        BindValue {
            view: binding.view,
            value: binding.value,
        }
    }
}

struct BindingSet<T: Copy> {
    local: Vec<BindValue<T>>,
    global: Option<BindValue<T>>,
}

impl<T: Copy> BindingSet<T> {
    fn new() -> Self {
        Self {
            local: vec![],
            global: None,
        }
    }

    fn bind(&mut self, binding: &Binding<T>) {
        let value = BindValue::from(binding);

        if binding.view.is_none() {
            self.global = Some(value);
        } else {
            let at = self.local.iter().position(|x| x.view == binding.view);

            match at {
                Some(i) => {
                    self.local[i] = value;
                }
                None => {
                    self.local.push(value);
                }
            }
        }
    }

    fn get(&self, view: Option<MonitorId>) -> Option<T> {
        let at = view.and_then(|id| self.local.iter().position(|x| x.view == Some(id)));

        match at {
            Some(i) => Some(self.local[i].value),
            None => self.global.as_ref().map(|x| x.value),
        }
    }
}

pub struct Keys<T: Copy> {
    root: x::Window,
    keymap: KeyMap,
    num_lock: Modifier,
    caps_lock: Modifier,
    scroll_lock: Modifier,
    bindings: HashMap<(Modifier, Keycode, bool), BindingSet<T>>,
}

impl<T: Copy> Keys<T> {
    pub fn new(conn: &mut Connection<T>, root: x::Window) -> Result<Self, Error> {
        /* TODO: support refreshing mappings */
        let mut keymap = KeyMap::new(conn)?;

        let num_lock = keymap.mask(keysym::Num_Lock)?;
        let caps_lock = keymap.mask(keysym::Caps_Lock)?;
        let scroll_lock = keymap.mask(keysym::Scroll_Lock)?;

        Ok(Keys {
            root: root,
            keymap: keymap,
            num_lock: num_lock,
            caps_lock: caps_lock,
            scroll_lock: scroll_lock,
            bindings: HashMap::new(),
        })
    }

    #[inline]
    fn grab(&self, conn: &mut Connection<T>, modifiers: Modifier, keycode: Keycode) -> xcb::VoidCookieChecked {
        let m = unsafe { x::ModMask::from_bits_unchecked(modifiers.bits()) };

        conn.send_request_checked(&x::GrabKey {
            owner_events: true,
            grab_window: self.root,
            modifiers: m,
            key: keycode as u8,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        })
    }

    pub fn bind(&mut self, conn: &mut Connection<T>, binding: &Binding<T>) -> Result<(), Error> {
        let mut cookies = Vec::with_capacity(8);

        for kc in self.keymap.keycodes(binding.keysym) {

            match binding.press {
                Press::Press => {
                    self.bindings
                        .entry((binding.mask, kc, true))
                        .or_insert(BindingSet::new())
                        .bind(&binding);
                }
                Press::Release => {
                    self.bindings
                        .entry((binding.mask, kc, false))
                        .or_insert(BindingSet::new())
                        .bind(&binding);
                }
                Press::Both => {
                    self.bindings
                        .entry((binding.mask, kc, true))
                        .or_insert(BindingSet::new())
                        .bind(&binding);

                    self.bindings
                        .entry((binding.mask, kc, false))
                        .or_insert(BindingSet::new())
                        .bind(&binding);
                }
            }

            match binding.mask {
                Modifier::ANY => {
                    self.grab(conn, Modifier::ANY, kc);
                }
                _ => {
                    let mut cookie = self.grab(conn, binding.mask, kc);
                    cookies.push(cookie);
                    cookie = self.grab(conn, binding.mask | self.num_lock, kc);
                    cookies.push(cookie);
                    cookie = self.grab(conn, binding.mask | self.caps_lock, kc);
                    cookies.push(cookie);
                    cookie = self.grab(conn, binding.mask | self.scroll_lock, kc);
                    cookies.push(cookie);
                    cookie = self.grab(conn, binding.mask | self.caps_lock | self.num_lock, kc);
                    cookies.push(cookie);
                    cookie = self.grab(conn, binding.mask | self.scroll_lock | self.num_lock, kc);
                    cookies.push(cookie);
                    cookie = self.grab(conn, binding.mask | self.scroll_lock | self.caps_lock, kc);
                    cookies.push(cookie);
                    cookie = self.grab(conn, binding.mask | self.num_lock | self.scroll_lock | self.caps_lock, kc);
                    cookies.push(cookie);
                }
            }
        }

        for cookie in cookies {
            conn.check_request(cookie)?;
        }

        Ok(())
    }

    pub fn get(
        &self,
        focus: Option<MonitorId>,
        mask: x::KeyButMask,
        k: Keycode,
        press: bool,
    ) -> Option<T> {
        let mut modifiers = unsafe { Modifier::from_bits_unchecked(mask.bits()) };
        modifiers.remove(self.num_lock | self.caps_lock | self.scroll_lock);

        self.bindings
            .get(&(modifiers, k, press))
            .or_else(|| self.bindings.get(&(Modifier::ANY, k, press)))
            .and_then(|b| b.get(focus))
    }
}
