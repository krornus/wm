use crate::rect::Rect;
use crate::slab::{SlabMap, AsIndex};
use crate::tag::{TagSetMask, TagSelection, TagSetId};
use crate::manager::Connection;
use crate::error::Error;

use xcb::x;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct ClientId {
    id: usize,
}

#[derive(Debug)]
pub enum Mode {
    Layout,
    Transient,
    Floating,
    Fullscreen,
}

pub struct Client {
    size: Rect,
    mode: Mode,
    visible: bool,
    window: x::Window,
    mask: SlabMap<TagSetMask>,
}

impl Client {
    pub fn new(window: x::Window, size: Rect, mode: Mode) -> Self {
        Client {
            size: size,
            mode: mode,
            visible: false,
            window: window,
            mask: SlabMap::new(),
        }
    }

    pub fn show(&mut self, conn: &mut Connection, visible: bool) -> Result<(), Error> {
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

    pub fn focus(&mut self, conn: &mut Connection) -> Result<(), Error> {
        let cookie = conn.send_request_checked(&x::SetInputFocus {
            revert_to: x::InputFocus::PointerRoot,
            focus: self.window,
            time: x::CURRENT_TIME,
        });

        conn.check_request(cookie)?;

        Ok(())
    }

    pub fn resize(&mut self, conn: &mut Connection, size: &Rect) -> Result<(), Error> {
        if &self.size != size {
            self.size = *size;

            let cookie = conn.send_request_checked(&x::ConfigureWindow {
                window: self.window,
                value_list: &[
                    x::ConfigWindow::X(self.size.x as i32),
                    x::ConfigWindow::Y(self.size.y as i32),
                    x::ConfigWindow::Width(self.size.w as u32),
                    x::ConfigWindow::Height(self.size.h as u32),
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
                    if !tagset.mask().visible(&TagSetMask::new()) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn insert_mask(&mut self, id: TagSetId, mask: TagSetMask) {
        self.mask.insert(id.as_index(), mask);
    }

    pub fn mask(&self) -> &SlabMap<TagSetMask> {
        &self.mask
    }

    pub fn mask_mut(&mut self) -> &mut SlabMap<TagSetMask> {
        &mut self.mask
    }
}
