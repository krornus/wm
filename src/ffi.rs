#[allow(non_camel_case_types)]

pub mod keyboard {
    use xcb::ffi::xcb_connection_t;

    pub type xcb_keysym_t = u32;
    pub type xcb_keycode_t = u32;

    pub enum xcb_key_symbols_t {}

    #[link(name = "xcb-keysyms")]
    extern "C" {
        pub fn xcb_key_symbols_alloc(conn: *mut xcb_connection_t) -> *mut xcb_key_symbols_t;
        pub fn xcb_key_symbols_free(syms: *mut xcb_key_symbols_t);

        pub fn xcb_key_symbols_get_keycode(
            syms: *mut xcb_key_symbols_t,
            keysym: xcb_keysym_t) -> *mut xcb_keycode_t;
    }
}
