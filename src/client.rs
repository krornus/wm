use crate::rect::Rect;
use crate::slab::{SlabMap, AsIndex};
use crate::tag::{TagMask, TagSelection, TagSetId};
use crate::wm::Connection;
use crate::error::Error;

use xcb::x;

pub struct Client {
    window: x::Window,
    visible: bool,
    rect: Rect,
    mask: SlabMap<TagMask>,
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
            mask: SlabMap::new(),
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

    pub fn masked<'a, 'b>(&self, sel: &TagSelection<'a, 'b>) -> bool {
        for (id, tagset) in sel.iter() {
            match self.mask.get(id.as_index()) {
                Some(m) => {
                    if !tagset.mask().visible(m) {
                        return false;
                    }
                }
                None => {
                    if !tagset.mask().visible(&TagMask::new()) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn insert_mask(&mut self, id: TagSetId, mask: TagMask) {
        self.mask.insert(id.as_index(), mask);
    }

    pub fn mask(&self) -> &SlabMap<TagMask> {
        &self.mask
    }

    pub fn mask_mut(&mut self) -> &mut SlabMap<TagMask> {
        &mut self.mask
    }
}
