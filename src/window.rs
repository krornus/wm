use crate::tree;
use crate::wm::Adapter;
use crate::rect::Rect;
use crate::client::Client;
use crate::layout::{Cell, Layout};

use xcb::x;

pub enum Window {
    Client(Client),
    Layout(Box<dyn Layout>),
}

pub struct WindowTree {
    tree: tree::Tree<Window>,
    focus: usize,
}

impl WindowTree {
    pub fn new(layout: impl Layout + 'static) -> Self {
        let win = Window::Layout(Box::new(layout));
        let tree = tree::Tree::new(win);
        let root = tree.root();

        WindowTree {
            tree: tree,
            focus: root,
        }
    }

    #[inline]
    pub fn root(&self) -> usize {
        self.tree.root()
    }

    #[inline]
    pub fn focus(&self) -> usize {
        self.focus
    }

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

    pub fn insert(&mut self, id: usize, value: Window) -> usize {
        /* TODO: fix this so that the type system only allows inserting into a layout */
        match self.tree.get(id).value {
            Window::Client(_) => { panic!("cannot add children to client") },
            _ => {},
        }

        self.tree.insert(id, value)
    }

    pub fn show(&mut self, adapter: &mut Adapter, index: usize, visible: bool) {
        let mut node = self.tree.get_mut(index);

        match node.value {
            Window::Client(ref mut client) => {
                return client.show(adapter, visible);
            },
            _ => {},
        }

        let mut child = node.child();

        while let Some(id) = child {
            /* get everything from node at the start in order to drop it for
             * lexical scoping to take effect, allowing us to recurse */
            node = self.tree.get_mut(id);
            child = node.next_sibling();

            match node.value {
                Window::Client(ref mut client) => {
                    client.show(adapter, visible);
                },
                Window::Layout(_) => {
                    let id = node.index();
                    self.show(adapter, id, visible);
                },
            }
        }

    }

    pub fn arrange(&mut self, adapter: &mut Adapter, index: usize, rect: &Rect) {
        let mut cells = vec![];

        let parent = self.tree.get(index);
        let mut child = parent.child();
        let mut node;

        while let Some(i) = child {
            node = self.tree.get(i);
            child = node.next_sibling();

            if let Window::Client(ref c) = node.value {
                cells.push(Cell::from(c));
            } else {
                cells.push(Cell::Hide);
            }
        }

        let mut node = self.tree.get_mut(index);

        match node.value {
            Window::Layout(ref mut layout) => {
                layout.arrange(rect, &mut cells);
            },
            _ => {
            }
        }

        let mut i = 0;
        let mut child = self.tree.get(index).child();

        while let Some(id) = child {
            node = self.tree.get_mut(id);
            child = node.next_sibling();

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
                Window::Layout(_) => {
                    let id = node.index();

                    /* node is dropped here via lexical scoping. */
                    match &cells[i] {
                        Cell::Hide => {
                            self.show(adapter, id, false);
                        },
                        Cell::Show(r) => {
                            self.arrange(adapter, id, r)
                        },
                        Cell::Focus(r) => {
                            self.focus = id;
                            self.arrange(adapter, id, r)
                        },
                    }
                },
            }

            i += 1;
        }
    }

    pub fn take(&mut self, mut other: WindowTree) {
        let root = other.tree.root();
        let children: Vec<_> = other.tree.children(root)
            .collect();

        let parent = self.tree.root();

        for child in children.into_iter().rev() {
            self.tree.take(&mut other.tree, child, parent);
        }
    }
}
