use crate::client::Client;
use crate::error::Error;
use crate::layout::{Cell, Layout};
use crate::rect::Rect;
use crate::tag::TagSelection;
use crate::tree;
use crate::wm::Connection;

use xcb::x;

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
        let mut tree = tree::Tree::new();
        tree.swap_root(win);

        WindowTree {
            tree: tree,
        }
    }

    #[inline]
    pub fn root(&self) -> usize {
        self.tree.root().unwrap()
    }

    pub fn find(&self, window: x::Window) -> Option<usize> {
        self.tree.iter().find_map(|(id, node)| match node.value {
            Window::Client(ref c) => {
                if window == c.window() {
                    Some(id)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    pub fn insert(&mut self, mut id: usize, value: Window) -> usize {
        /* TODO: fix this so that the type system only allows inserting into a layout */
        let node = self.tree.get(id);
        match node.value {
            Window::Client(_) => {
                id = node
                    .parent()
                    .expect("cannot add node to single-client tree");
            }
            _ => {}
        }

        self.tree.insert(id, value)
    }

    pub fn get(&self, id: usize) -> Option<&Client> {
        match self.tree.get(id).value {
            Window::Client(ref client) => Some(client),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Client> {
        match self.tree.get_mut(id).value {
            Window::Client(ref mut client) => Some(client),
            _ => None,
        }
    }

    pub fn next(&self, id: usize) -> Option<usize> {
        self.tree.get(id).next_sibling()
    }

    pub fn previous(&self, id: usize) -> Option<usize> {
        self.tree.get(id).previous_sibling()
    }

    pub fn remove(&mut self, id: usize) -> Client {
        let node = self.tree.drop(id);

        match node.value {
            Window::Client(c) => c,
            Window::Layout(_) => panic!("attempt to remove layout"),
        }
    }

    pub fn show<T>(&mut self, conn: &mut Connection<T>, index: usize, visible: bool) -> Result<(), Error> {
        let mut node = self.tree.get_mut(index);

        match node.value {
            Window::Client(ref mut client) => {
                return client.show(conn, visible);
            }
            _ => {}
        }

        let mut child = node.first_child();

        while let Some(id) = child {
            /* get everything from node at the start in order to drop it for
             * lexical scoping to take effect, allowing us to recurse */
            node = self.tree.get_mut(id);
            child = node.next_sibling();

            match node.value {
                Window::Client(ref mut client) => {
                    client.show(conn, visible)?;
                }
                Window::Layout(_) => {
                    let id = node.index();
                    self.show(conn, id, visible)?;
                }
            }
        }

        Ok(())
    }

    pub fn arrange<'a, 'b, T>(
        &mut self,
        conn: &mut Connection<T>,
        mask: &TagSelection<'a, 'b>,
        rect: &Rect,
    ) -> Result<Option<usize>, Error> {
        if let Some(root) = self.tree.root() {
            let cookie = conn.send_request(&x::GetInputFocus {});

            let masktree = MaskTree::new(conn, self, mask, root)?;

            let reply = conn.wait_for_reply(cookie)?;

            match masktree.root() {
                Some(root) => {
                    self.arrange_recursive(conn, &masktree, root, rect, reply.focus())
                }
                None => {
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }

    fn arrange_recursive<T>(
        &mut self,
        conn: &mut Connection<T>,
        masktree: &MaskTree,
        index: usize,
        rect: &Rect,
        active: x::Window,
    ) -> Result<Option<usize>, Error> {

        let mut focus = None;

        let mut cells = vec![];
        let parent = masktree.get(index);
        let mut child = parent.first_child();

        while let Some(id) = child {
            let node = masktree.get(id);
            let window = self.tree.get(node.value);

            child = node.next_sibling();

            if let Window::Client(ref c) = window.value {
                if c.window() == active {
                    cells.push(Cell::Focus(*c.rect()));
                } else {
                    cells.push(Cell::Show(*c.rect()));
                }
            } else {
                cells.push(Cell::Hide);
            }
        }

        let node = masktree.get(index);
        let window = self.tree.get_mut(node.value);

        match window.value {
            Window::Layout(ref mut layout) => {
                layout.arrange(rect, &mut cells);
            }
            _ => {}
        }

        let mut i = 0;
        child = parent.first_child();

        while let Some(id) = child {
            let node = masktree.get(id);
            let window = self.tree.get_mut(node.value);

            child = node.next_sibling();

            match window.value {
                Window::Client(ref mut client) => match &cells[i] {
                    Cell::Hide => {
                        client.show(conn, false)?;
                    }
                    Cell::Show(r) => {
                        client.show(conn, true)?;
                        client.resize(conn, r)?;
                    }
                    Cell::Focus(r) => {
                        focus = Some(i);
                        client.focus(conn)?;
                        client.show(conn, true)?;
                        client.resize(conn, r)?;
                    }
                },
                Window::Layout(_) => {
                    /* node is dropped here via lexical scoping. */
                    match &cells[i] {
                        Cell::Hide => {
                            self.show(conn, node.value, false)?;
                        }
                        Cell::Show(r) => {
                            focus = self.arrange_recursive(conn, masktree, id, r, active)?;
                        }
                        Cell::Focus(r) => {
                            focus = self.arrange_recursive(conn, masktree, id, r, active)?;
                        }
                    }
                }
            }

            i += 1;
        }

        Ok(focus)
    }

    pub fn take(&mut self, mut other: WindowTree) {
        let root = other.root();
        let children: Vec<_> = other.tree.children(root).collect();

        let parent = self.root();

        for child in children.into_iter().rev() {
            self.tree.take(&mut other.tree, child, parent);
        }
    }
}

struct MaskTree {
    tree: tree::Tree<usize>,
}

impl MaskTree {
    fn new<'a, 'b, T>(
        conn: &mut Connection<T>,
        win: &mut WindowTree,
        mask: &TagSelection<'a, 'b>,
        index: usize,
    ) -> Result<MaskTree, Error> {
        let mut tree = MaskTree {
            tree: tree::Tree::new(),
        };

        let root = tree.generate(conn, win, mask, index)?;
        tree.tree.set_root(root);

        Ok(tree)
    }

    fn root(&self) -> Option<usize> {
        self.tree.root()
    }

    fn generate<'a, 'b, T>(
        &mut self,
        conn: &mut Connection<T>,
        tree: &mut WindowTree,
        mask: &TagSelection<'a, 'b>,
        from: usize,
    ) -> Result<Option<usize>, Error> {
        /* construct a tree, bottom up, such that any nodes of the tree
         * which are masked out are excluded from the final product */
        let mut node = tree.tree.get_mut(from);

        match node.value {
            Window::Client(ref mut client) => {
                if client.mask(mask) {
                    Ok(Some(self.tree.orphan(from)))
                } else {
                    client.show(conn, false)?;
                    Ok(None)
                }
            }
            Window::Layout(_) => {
                let mut children = false;
                let parent = self.tree.orphan(from);
                let mut child = node.first_child();

                while let Some(id) = child {
                    node = tree.tree.get_mut(id);
                    child = node.next_sibling();

                    match self.generate(conn, tree, mask, id)? {
                        Some(orphan) => {
                            children = true;
                            self.tree.adopt(parent, orphan);
                        }
                        None => {}
                    }
                }

                if children {
                    Ok(Some(parent))
                } else {
                    self.tree.remove(parent);
                    Ok(None)
                }
            }
        }
    }

    #[inline]
    fn get(&self, index: usize) -> &tree::TreeNode<usize> {
        self.tree.get(index)
    }
}
