use std::collections::HashMap;

use crate::rect::Rect;
use crate::tag::{TagMask, TagSelection, TagSetId};
use crate::wm::Connection;
use crate::error::Error;

use xcb::x;

pub struct Client {
    window: x::Window,
    visible: bool,
    rect: Rect,
    mask: HashMap<TagSetId, TagMask>,
}

impl Client {
    #[inline]
    pub fn window(&self) -> x::Window {
        self.window
    }

    #[inline]
    pub fn visible(&self) -> bool {
        self.visible
    }

    #[inline]
    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}

impl Client {
    pub fn new(window: x::Window, rect: Rect) -> Self {
        Client {
            window: window,
            visible: false,
            rect: rect,
            mask: HashMap::new(),
        }
    }

    pub fn show<T>(&mut self, conn: &mut Connection<T>, visible: bool) -> Result<(), Error> {
        if self.visible != visible {
            self.visible = visible;

            let cookie = if visible {
                conn.send_request_checked(&x::MapWindow {
                    window: self.window,
                })
            } else {
                conn.send_request_checked(&x::UnmapWindow {
                    window: self.window,
                })
            };

            conn.check_request(cookie)?;
        }

        Ok(())
    }

    pub fn focus<T>(&mut self, conn: &mut Connection<T>) -> Result<(), Error> {
        let cookie = conn.send_request_checked(&x::SetInputFocus {
            revert_to: x::InputFocus::PointerRoot,
            focus: self.window,
            time: x::CURRENT_TIME,
        });

        conn.check_request(cookie)?;

        Ok(())
    }

    pub fn resize<T>(&mut self, conn: &mut Connection<T>, rect: &Rect) -> Result<(), Error> {
        if &self.rect != rect {
            self.rect = *rect;

            let cookie = conn.send_request_checked(&x::ConfigureWindow {
                window: self.window,
                value_list: &[
                    x::ConfigWindow::X(self.rect.x as i32),
                    x::ConfigWindow::Y(self.rect.y as i32),
                    x::ConfigWindow::Width(self.rect.w as u32),
                    x::ConfigWindow::Height(self.rect.h as u32),
                ],
            });

            conn.check_request(cookie)?;
        }

        Ok(())
    }

    pub fn mask<'a, 'b>(&self, mask: &TagSelection<'a, 'b>) -> bool {
        for (id, mask) in mask.iter() {
            match self.mask.get(&id) {
                Some(m) => {
                    if !mask.visible(m) {
                        return false;
                    }
                }
                None => {
                    if !mask.visible(&TagMask::new()) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn set_mask(&mut self, mask: HashMap<TagSetId, TagMask>) {
        self.mask = mask.clone()
    }

    pub fn get_mask_mut(&mut self, id: TagSetId) -> Option<&mut TagMask> {
        self.mask.get_mut(&id)
    }
}
