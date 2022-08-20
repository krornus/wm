use std::collections::HashMap;

use crate::error::Error;
use crate::wm::Adapter;
use crate::display::ViewId;
use crate::keysym;

use xcb::x::{self, Keysym, Keycode};
use bitflags::bitflags;

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

    pub fn mask(&mut self, keysym: Keysym) -> Result<Modifier, Error> {
        for target in self.keycodes(keysym) {
            for (i, keycode) in self.modmap.keycodes().iter().enumerate() {
                if target == *keycode {
                    /* reply.keycodes really returns a 2 dimensional array,
                     *   keycodes[8][keycodes_per_modifier]
                     * by dividing the index by 8, we get the associated
                     * modifier, shifting it gives us the mask. */
                    return unsafe { Ok(Modifier::from_bits_unchecked(1 << (i / 8))) };
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
    pub view: Option<ViewId>,
    pub mask: Modifier,
    pub keysym: Keysym,
    pub press: Press,
    pub value: T,
}

#[derive(Clone)]
struct BindValue<T: Copy> {
    view: Option<ViewId>,
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
                },
                None => {
                    self.local.push(value);
                }
            }
        }
    }

    fn get(&self, view: Option<ViewId>) -> Option<T> {
        let at = view.and_then(|id| self.local.iter().position(|x| x.view == Some(id)));

        match at {
            Some(i) => {
                Some(self.local[i].value)
            },
            None => {
                self.global.as_ref().map(|x| x.value)
            }
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
    pub fn new(conn: &xcb::Connection, root: x::Window) -> Result<Self, Error> {
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
    fn grab(&self, adapter: &mut Adapter<T>, modifiers: Modifier, keycode: Keycode) {
        let m = unsafe { x::ModMask::from_bits_unchecked(modifiers.bits()) };
        println!("grab: {:?}: [{:?} + {:?}]", self.root, m, keycode);

        adapter.request(&x::GrabKey {
            owner_events: true,
            grab_window: self.root,
            modifiers: m,
            key: keycode as u8,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        })
    }

    pub fn bind(&mut self, adapter: &mut Adapter<T>, binding: &Binding<T>) -> Result<(), Error> {
        for kc in self.keymap.keycodes(binding.keysym) {
            match binding.press {
                Press::Press => {
                    self.bindings.entry((binding.mask, kc, true))
                        .or_insert(BindingSet::new())
                        .bind(&binding);
                },
                Press::Release => {
                    self.bindings.entry((binding.mask, kc, false))
                        .or_insert(BindingSet::new())
                        .bind(&binding);
                },
                Press::Both => {
                    self.bindings.entry((binding.mask, kc, true))
                        .or_insert(BindingSet::new())
                        .bind(&binding);

                    self.bindings.entry((binding.mask, kc, false))
                        .or_insert(BindingSet::new())
                        .bind(&binding);
                }
            }


            match binding.mask {
                Modifier::ANY => {
                    self.grab(adapter, Modifier::ANY, kc);
                },
                _ => {
                    self.grab(adapter, binding.mask, kc);
                    self.grab(adapter, binding.mask | self.num_lock, kc);
                    self.grab(adapter, binding.mask | self.caps_lock, kc);
                    self.grab(adapter, binding.mask | self.scroll_lock, kc);
                    self.grab(adapter, binding.mask | self.caps_lock | self.num_lock, kc);
                    self.grab(adapter, binding.mask | self.scroll_lock | self.num_lock, kc);
                    self.grab(adapter, binding.mask | self.scroll_lock | self.caps_lock, kc);
                    self.grab(adapter, binding.mask | self.num_lock | self.scroll_lock | self.caps_lock, kc);
                },
            }
        }

        Ok(())
    }

    pub fn get(&self, focus: Option<ViewId>, mask: x::KeyButMask, k: Keycode, press: bool) -> Option<T> {
        let mut modifiers = unsafe { Modifier::from_bits_unchecked(mask.bits()) };
        modifiers.remove(self.num_lock | self.caps_lock | self.scroll_lock);

        self.bindings
            .get(&(modifiers, k, press))
            .or_else(|| self.bindings.get(&(Modifier::ANY, k, press)))
            .and_then(|b| b.get(focus))
    }
}
