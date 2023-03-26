use xcb::x::{Window, Keycode, Keysym};

use crate::rect::Point;
use crate::client::ClientId;
use crate::monitor::MonitorId;
use crate::pane::PaneId;
use crate::keyboard::{KeyPress, KeyModifier};

#[derive(Debug)]
pub enum Event {
    MonitorConnect { monitor: MonitorId, x: i16, y: i16, width: u16, height: u16, },
    MonitorDisconnect { monitor: MonitorId, },
    MonitorPrimary { monitor: MonitorId, },
    MonitorSize { monitor: MonitorId, x: i16, y: i16, width: u16, height: u16, },
    ClientCreate { client: ClientId, x: i16, y: i16, width: u16, height: u16, },
    ClientSize { client: ClientId, x: i16, y: i16, width: u16, height: u16, },
    ClientShow { client: ClientId, group: PaneId, },
    ClientHide { client: ClientId, },
    ClientExpose { client: ClientId, },
    ClientDestroy { client: ClientId, },
    ClientPane { client: ClientId, group: PaneId, },
    ClientEnter { client: ClientId, },
    ClientLeave { client: ClientId, },
    ClientKey { client: ClientId, keypress: KeyPress, keysym: Keysym, keymod: KeyModifier, },
    ClientUrgent { client: ClientId, urgent: bool, },
    ClientFullscreen { client: ClientId, fullscreen: bool, },
    Key { root: Window, keysym: Keysym, mask: KeyModifier, press: bool },
}
