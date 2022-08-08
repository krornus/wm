use crate::rect::Rect;
use crate::tag::TagIndex;
use crate::wm::Adapter;

use xcb::x;

pub struct Client {
    pub window: x::Window,
    pub visible: bool,
    pub focus: bool,
    pub index: TagIndex,
    pub rect: Rect,
}

impl Client {
    pub fn focus(&mut self, p: bool) {
        if self.focus != p {
            /* TODO: actually grab/release focus */
            self.focus = p;
        }
    }

    pub fn resize(&mut self, adapter: &mut Adapter, rect: Rect) {
        if self.rect != rect {
            self.rect = rect;

            adapter.request(&x::ConfigureWindow {
                window: self.window,
                value_list: &[
                    x::ConfigWindow::X(self.rect.x as i32),
                    x::ConfigWindow::Y(self.rect.y as i32),
                    x::ConfigWindow::Width(self.rect.w as u32),
                    x::ConfigWindow::Height(self.rect.h as u32),
                ],
            });
        }
    }
}
