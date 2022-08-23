use std::collections::HashMap;

use crate::rect::Rect;
use crate::tag::{TagMask, TagSelection, TagSetId};
use crate::wm::Adapter;

use xcb::x;

pub struct Client {
    window: x::Window,
    visible: bool,
    focus: bool,
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
    pub fn focused(&self) -> bool {
        self.focus
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
            focus: false,
            rect: rect,
            mask: HashMap::new(),
        }
    }

    pub fn show<T>(&mut self, adapter: &mut Adapter<T>, visible: bool) {
        if self.visible != visible {
            self.visible = visible;

            if visible {
                adapter.request(&x::MapWindow {
                    window: self.window,
                });
            } else {
                adapter.request(&x::UnmapWindow {
                    window: self.window,
                });
            }
        }
    }

    pub fn focus<T>(&mut self, _: &mut Adapter<T>, p: bool) {
        if self.focus != p {
            /* TODO: actually grab/release focus */
            self.focus = p;
        }
    }

    pub fn resize<T>(&mut self, adapter: &mut Adapter<T>, rect: &Rect) {
        if &self.rect != rect {
            self.rect = *rect;

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
