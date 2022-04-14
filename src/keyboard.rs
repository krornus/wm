use xkbcommon::xkb;

use crate::error::Error;

pub struct Keyboard {
    state: xkb::State,
}

impl Keyboard {
    pub fn new(conn: &xcb::Connection) -> Result<Self, Error> {
        Self::from_id(conn, Self::core_id(conn)?)
    }

    pub fn from_id(conn: &xcb::Connection, id: i32) -> Result<Self, Error> {
        Self::select(conn)?;

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap =
            xkb::x11::keymap_new_from_device(&context, conn, id, xkb::KEYMAP_COMPILE_NO_FLAGS);

        let state = xkb::x11::state_new_from_device(&keymap, conn, id);

        Ok(Keyboard { state: state })
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
        let events = xcb::xkb::EventType::NEW_KEYBOARD_NOTIFY
                   | xcb::xkb::EventType::MAP_NOTIFY
                   | xcb::xkb::EventType::STATE_NOTIFY;

        let map_parts = xcb::xkb::MapPart::KEY_TYPES
                      | xcb::xkb::MapPart::KEY_SYMS
                      | xcb::xkb::MapPart::MODIFIER_MAP
                      | xcb::xkb::MapPart::EXPLICIT_COMPONENTS
                      | xcb::xkb::MapPart::KEY_ACTIONS
                      | xcb::xkb::MapPart::KEY_BEHAVIORS
                      | xcb::xkb::MapPart::VIRTUAL_MODS
                      | xcb::xkb::MapPart::VIRTUAL_MOD_MAP;

        let spec = unsafe { std::mem::transmute::<_, u32>(xcb::xkb::Id::UseCoreKbd) };

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

    pub fn keysym(&self, ev: &xcb::x::KeyPressEvent) -> xkbcommon::xkb::Keysym {
        self.state.key_get_one_sym(ev.detail() as u32)
    }
}
