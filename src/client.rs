use crate::rect::Rect;
use crate::tag::TagIndex;

use xcb::x;

pub struct Client {
    pub scope: Rect,
    pub window: x::Window,
    pub visible: bool,
    index: TagIndex,
}
