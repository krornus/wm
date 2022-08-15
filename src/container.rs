use crate::tree;
use crate::wm::Adapter;
use crate::rect::Rect;
use crate::client::Client;
use crate::layout::{Cell, Layout};
use crate::slab::SlabIndex;

use xcb::x;

pub type WindowId = SlabIndex;

pub enum Window {
    Client(Client),
    Layout(Box<dyn Layout>),
}

pub struct WindowTree {
    tree: tree::Tree<Window>,
}

impl WindowTree {
    pub fn new(layout: impl Layout + 'static) -> Self {
        let win = Window::Layout(Box::new(layout));
        WindowTree {
            tree: tree::Tree::new(win)
        }
    }

    #[inline]
    pub fn root(&self) -> WindowId {
        self.tree.root()
    }

    #[inline]
    pub fn get_client(&self, window: x::Window) -> Option<&Client> {
        self.tree.iter().find_map(|win| match win {
            Window::Client(ref c) => if window == c.window() {
                Some(c)
            } else {
                None
            },
            _ => {
                None
            }
        })
    }

    #[inline]
    pub fn get_client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.tree.iter_mut().find_map(|win| match win {
            Window::Client(ref mut c) => if window == c.window() {
                Some(c)
            } else {
                None
            },
            _ => {
                None
            }
        })
    }

    #[inline]
    pub fn insert(&mut self, id: &WindowId, value: Window) -> WindowId {
        /* TODO: fix this so that the type system only allows inserting into a layout */
        self.tree.insert(id, value).unwrap()
    }

    // pub fn take(&mut self, mut other: Container) {
    //     let root = other.tree.root();
    //     let children: Vec<_> = other.tree.children(root)
    //         .collect();

    //     let parent = self.tree.root();

    //     for child in children.into_iter().rev() {
    //         self.tree.take(&mut other.tree, &child, &parent);
    //     }
    // }

    pub fn arrange(&mut self, adapter: &mut Adapter, index: &WindowId, rect: &Rect) {
        let mut cells = vec![];

        let parent = self.tree.get(index).unwrap();
        let mut child = parent.child();
        let mut node;

        while let Some(i) = child {
            node = self.tree.get(&i).unwrap();
            child = node.right();
            cells.push(Cell::Hide);
        }

        let mut node = self.tree.get_mut(index).unwrap();

        match node.value {
            Window::Layout(ref mut layout) => {
                layout.arrange(adapter, rect, &mut cells);
            },
            _ => {
            }
        }

        let mut i = 0;
        let mut child = self.tree.get(index).unwrap().child();

        while let Some(ref id) = child {
            let node = self.tree.get_mut(&id).unwrap();
            child = node.right();

            match node.value {
                Window::Client(ref mut client) => {
                    match &cells[i] {
                        Cell::Hide => {
                            client.show(adapter, false);
                        },
                        Cell::Show(r) => {
                            client.show(adapter, true);
                            client.resize(adapter, r);
                        },
                        Cell::Focus(r) => {
                            client.show(adapter, true);
                            client.resize(adapter, r);
                        },
                    }
                },
                Window::Layout(ref mut layout) => {
                    match &cells[i] {
                        Cell::Show(r) => {
                            let x = node.index();
                            self.arrange(adapter, &x, r)
                        },
                        _ => {},
                    }
                },
            }

            i += 1;
        }
    }

    pub fn show(&mut self, adapter: &mut Adapter, visible: bool) {
    }

    // #[inline]
    // pub fn get(&self, id: &WindowId) -> Option<&Scope> {
    //     self.tree.get(id).map(|n| &n.value)
    // }

    // #[inline]
    // pub fn get_mut(&mut self, id: &WindowId) -> Option<&mut Scope> {
    //     self.tree.get_mut(id).map(|n| &mut n.value)
    // }
}
