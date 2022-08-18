use std::collections::HashMap;

use crate::tree;
use crate::wm::Adapter;
use crate::tag::{TagSetId, TagMask};
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
        let mut tree = tree::Tree::new();
        tree.swap_root(win);
        let root = tree.root().unwrap();

        WindowTree {
            tree: tree,
            focus: root,
        }
    }

    #[inline]
    pub fn root(&self) -> usize {
        self.tree.root().unwrap()
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

    pub fn show<T>(&mut self, adapter: &mut Adapter<T>, index: usize, visible: bool) {
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

    pub fn arrange<T>(&mut self, adapter: &mut Adapter<T>, mask: &HashMap<TagSetId, TagMask>, index: usize, rect: &Rect) {
        let masktree = MaskTree::new(adapter, self, mask, index);

        match masktree.root() {
            Some(root) => self.arrange_recursive(adapter, &masktree, root, rect),
            None => {},
        }
    }

    fn arrange_recursive<T>(&mut self, adapter: &mut Adapter<T>, masktree: &MaskTree, index: usize, rect: &Rect) {
        let mut cells = vec![];
        let parent = masktree.get(index);
        let mut child = parent.child();

        while let Some(id) = child {
            let node = masktree.get(id);
            let window = self.tree.get(node.value);

            child = node.next_sibling();

            if let Window::Client(ref c) = window.value {
                cells.push(Cell::from(c));
            } else {
                cells.push(Cell::Hide);
            }
        }

        let node = masktree.get(index);
        let window = self.tree.get_mut(node.value);

        match window.value {
            Window::Layout(ref mut layout) => {
                layout.arrange(rect, &mut cells);
            },
            _ => {
            }
        }

        let mut i = 0;
        child = parent.child();

        while let Some(id) = child {
            let node = masktree.get(id);
            let window = self.tree.get_mut(node.value);

            child = node.next_sibling();

            match window.value {
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
                    /* node is dropped here via lexical scoping. */
                    match &cells[i] {
                        Cell::Hide => {
                            self.show(adapter, node.value, false);
                        },
                        Cell::Show(r) => {
                            self.arrange_recursive(adapter, masktree, id, r)
                        },
                        Cell::Focus(r) => {
                            self.focus = id;
                            self.arrange_recursive(adapter, masktree, id, r)
                        },
                    }
                },
            }

            i += 1;
        }
    }

    pub fn take(&mut self, mut other: WindowTree) {
        let root = other.root();
        let children: Vec<_> = other.tree.children(root)
            .collect();

        let parent = self.root();

        for child in children.into_iter().rev() {
            self.tree.take(&mut other.tree, child, parent);
        }
    }
}

pub struct MaskTree {
    tree: tree::Tree<usize>,
}

impl MaskTree {
    fn new<T>(adapter: &mut Adapter<T>, win: &mut WindowTree, mask: &HashMap<TagSetId, TagMask>, index: usize) -> MaskTree {
        let mut tree = MaskTree {
            tree: tree::Tree::new()
        };

        let root = tree.generate(adapter, win, mask, index);
        tree.tree.set_root(root);

        tree
    }

    fn root(&self) -> Option<usize> {
        self.tree.root()
    }

    fn generate<T>(&mut self, adapter: &mut Adapter<T>, tree: &mut WindowTree, mask: &HashMap<TagSetId, TagMask>, from: usize) -> Option<usize> {
        /* construct a tree, bottom up, such that any nodes of the tree
         * which are masked out are excluded from the final product */
        let mut node = tree.tree.get_mut(from);

        match node.value {
            Window::Client(ref mut client) => {
                if client.mask(mask) {
                    println!("Unhidden window: {:?}", client.window());
                    Some(self.tree.orphan(from))
                } else {
                    println!("Hidden window: {:?}", client.window());
                    client.show(adapter, false);
                    None
                }
            },
            Window::Layout(_) => {
                let mut children = false;
                let parent = self.tree.orphan(from);
                let mut child = node.child();

                while let Some(id) = child {
                    node = tree.tree.get_mut(id);
                    child = node.next_sibling();

                    match self.generate(adapter, tree, mask, id) {
                        Some(orphan) => {
                            children = true;
                            self.tree.adopt(parent, orphan);
                        },
                        None => {},
                    }
                }

                if children {
                    Some(parent)
                } else {
                    self.tree.remove(parent);
                    None
                }

            },
        }
    }

    #[inline]
    fn get(&self, index: usize) -> &tree::TreeNode<usize> {
        self.tree.get(index)
    }
}
