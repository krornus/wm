use std::ops::{Index, IndexMut};

use crate::client::Client;
use crate::error::Error;
use crate::layout::{Cell, Layout};
use crate::rect::Rect;
use crate::tag::TagSelection;
use crate::tree;
use crate::wm::Connection;

use xcb::x;

/// WindowTree is the main Tree which contains all Clients for a monitor, along
/// with their respective layouts.
pub struct WindowTree {
    tree: tree::Tree<Window>,
}

/// Window is the node type of the window tree. It can either be a Client,
/// which is always a leaf, or a struct implementing the Layout trait. Layouts
/// may contain other Window children.
pub enum Window {
    Client(Client),
    Layout(Box<dyn Layout>),
}

/// A ClientId is the index of a Client in a WindowTree
#[repr(transparent)]
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ClientId {
    inner: usize
}

/// A LayoutId is the index of a Layout in a WindowTree
#[repr(transparent)]
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct LayoutId {
    inner: usize
}

/// MaskTree is a secondary tree that is recursively generated from a WindowTree
/// during window arangement. It is generated from the combination of the
/// WindowTree and a TagSelection, and represents the tree of currently visible
/// elements.
struct MaskTree {
    tree: tree::Tree<usize>,
}

impl MaskTree {
    /// Creates a new MaskTree struct. Any clients not present within the
    /// MaskTree are hidden from view. Empty layouts are removed from the tree,
    /// so any leafs are guarenteed to be Clients.
    fn new<'a, 'b, T>(
        conn: &mut Connection<T>,
        win: &mut WindowTree,
        mask: &TagSelection<'a, 'b>,
        index: usize,
    ) -> Result<MaskTree, Error> {

        let mut tree = MaskTree {
            tree: tree::Tree::new(),
        };

        /* begin recursive step */
        let root = tree.generate(conn, win, mask, index)?;

        tree.tree.set_root(root);

        Ok(tree)
    }

    /// Performs the actual generation of the tree. Called during creation by
    /// MaskTree::mask(). Returns the root index if one is present, or an
    /// error.
    ///
    /// conn is the Connection to the xcb server, tree is the tree to mask,
    /// mask is the mask to apply to the tree, and from is the current index in
    /// the WindowTree from which to generate nodes.
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

        /* we mutate a single node to satisfy the borrow checker */
        match node.value {
            /* Clients are guarenteed to be leafs */
            Window::Client(ref mut client) => {
                if client.mask(mask) {
                    Ok(Some(self.tree.orphan(from)))
                } else {
                    /* this is our only chance to hide this window, as it is
                     * masked out after this point */
                    client.show(conn, false)?;
                    Ok(None)
                }
            }
            /* Layouts may or may not be leafs. handle recursively */
            Window::Layout(_) => {
                let mut children = false;
                /* assume we will have children, we can undo this later */
                let parent = self.tree.orphan(from);
                let mut child = node.first_child();

                /* for each child, generate a new tree and adopt it, resulting
                 * in a bottom-up construction. we do bottom-up in order to
                 * discard empty layouts from the final product */
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
                    /* no children - remove the parent */
                    self.tree.remove(parent);
                    Ok(None)
                }
            }
        }
    }

    /// Get an immutable reference from the MaskTree
    #[inline]
    fn get(&self, index: usize) -> &tree::TreeNode<usize> {
        self.tree.get(index)
    }

    /// Get the root index of the MaskTree
    #[inline]
    fn root(&self) -> Option<usize> {
        self.tree.root()
    }

}

impl WindowTree {
    /// Create a new window tree with a base layout
    pub fn new(layout: impl Layout + 'static) -> Self {
        let win = Window::Layout(Box::new(layout));
        let mut tree = tree::Tree::new();
        tree.swap_root(win);

        WindowTree {
            tree: tree,
        }
    }

    /// Gets the root layout identifier
    #[inline]
    pub fn root(&self) -> LayoutId {
        LayoutId {
            inner: self.tree.root().unwrap()
        }
    }

    /// Insert a client into the tree, returning its identifier
    #[inline]
    pub fn client(&mut self, id: LayoutId, client: Client) -> ClientId {
        ClientId {
            inner: self.tree.insert(id.inner, Window::Client(client))
        }
    }

    /// Insert a layout into the tree, returning its identifier
    #[inline]
    pub fn layout(&mut self, id: LayoutId, layout: impl Layout + 'static) -> LayoutId {
        LayoutId {
            inner: self.tree.insert(id.inner, Window::Layout(Box::new(layout)))
        }
    }

    /// Search for a client in the tree based on its window
    pub fn find(&self, window: x::Window) -> Option<ClientId> {
        self.tree.iter().find_map(|(id, node)| match node.value {
            Window::Client(ref c) => {
                if window == c.window() {
                    Some(ClientId::from(id))
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    /// Remove and return a node from the tree
    pub fn remove<I: AsRawIndex>(&mut self, id: I) -> Window {
        self.tree.prune(id.as_raw()).value
    }

    /// Show or hide an entire layout
    pub fn show<T>(&mut self, conn: &mut Connection<T>, index: LayoutId, visible: bool) -> Result<(), Error> {
        let mut node = self.tree.get_mut(index.inner);

        match node.value {
            Window::Client(_) => {
                panic!("WindowTree: invalid layout id");
            }
            _ => {
            }
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
                    self.show(conn, LayoutId::from(id), visible)?;
                }
            }
        }

        Ok(())
    }

    /// Arrange the windows in this tree, given a tag mask and a containing rectangle
    pub fn arrange<'a, 'b, T>(
        &mut self,
        conn: &mut Connection<T>,
        mask: &TagSelection<'a, 'b>,
        rect: &Rect,
    ) -> Result<Option<ClientId>, Error> {
        if let Some(root) = self.tree.root() {
            /* request input focus, needed for Layout::arrange cells */
            let cookie = conn.send_request(&x::GetInputFocus {});
            /* generate the tree of actually visible clients/layouts */
            let masktree = MaskTree::new(conn, self, mask, root)?;

            let reply = conn.wait_for_reply(cookie)?;

            match masktree.root() {
                Some(root) => {
                    /* there is at least one window present -- arrange it */
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
    ) -> Result<Option<ClientId>, Error> {

        let mut focus = None;

        let mut cells = vec![];
        let parent = masktree.get(index);
        let mut child = parent.first_child();

        /* loop all children and create a Cell element for each one */
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

        /* if this is a layout, pass the cell array and the containing
         * rectangle to the layout trait. this will modify the cells in place */
        let node = masktree.get(index);
        let window = self.tree.get_mut(node.value);

        match window.value {
            Window::Layout(ref mut layout) => {
                layout.arrange(rect, &mut cells);
            }
            _ => {}
        }

        /* now the cells array is ready to be applied to the windows */
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
                        focus = Some(ClientId::from(i));
                        client.focus(conn)?;
                        client.show(conn, true)?;
                        client.resize(conn, r)?;
                    }
                },
                Window::Layout(_) => {
                    /* node is dropped here via lexical scoping. */
                    match &cells[i] {
                        Cell::Hide => {
                            self.show(conn, LayoutId::from(node.value), false)?;
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
        let children: Vec<_> = other.tree.children(root.inner).collect();

        let parent = self.root();

        for child in children.into_iter().rev() {
            self.tree.take(&mut other.tree, child, parent.inner);
        }
    }
}


pub trait AsRawIndex {
    fn as_raw(&self) -> usize;
}

impl AsRawIndex for ClientId {
    fn as_raw(&self) -> usize {
        self.inner
    }
}

impl AsRawIndex for LayoutId {
    fn as_raw(&self) -> usize {
        self.inner
    }
}


impl WindowTree {
    pub fn parent<I: AsRawIndex>(&self, i: I) -> Option<LayoutId> {
        self.tree.get(i.as_raw()).parent()
            .map(|x| LayoutId::from(x))
    }

    pub fn next_client<I: AsRawIndex>(&self, i: I) -> Option<ClientId> {
        let mut node = self.tree.get(i.as_raw());

        loop {
            let id = node.next_sibling()?;
            node = self.tree.get(id);

            match node.value {
                Window::Client(_) => {
                    break Some(ClientId::from(id));
                }
                _ => {
                    continue;
                }
            }
        }
    }

    pub fn previous_client<I: AsRawIndex>(&self, i: I) -> Option<ClientId> {
        let mut node = self.tree.get(i.as_raw());

        loop {
            let id = node.previous_sibling()?;
            node = self.tree.get(id);

            match node.value {
                Window::Client(_) => {
                    break Some(ClientId::from(id));
                }
                _ => {
                    continue;
                }
            }
        }
    }

    pub fn next_layout<I: AsRawIndex>(&self, i: I) -> Option<LayoutId> {
        let mut node = self.tree.get(i.as_raw());

        loop {
            let id = node.next_sibling()?;
            node = self.tree.get(id);

            match node.value {
                Window::Layout(_) => {
                    break Some(LayoutId::from(id));
                }
                _ => {
                    continue;
                }
            }
        }
    }

    pub fn previous_layout<I: AsRawIndex>(&self, i: I) -> Option<LayoutId> {
        let mut node = self.tree.get(i.as_raw());

        loop {
            let id = node.previous_sibling()?;
            node = self.tree.get(id);

            match node.value {
                Window::Layout(_) => {
                    break Some(LayoutId::from(id));
                }
                _ => {
                    continue;
                }
            }
        }
    }
}


impl From<usize> for ClientId {
    #[inline]
    fn from(i: usize) -> Self {
        ClientId {
            inner: i
        }
    }
}

impl From<usize> for LayoutId {
    #[inline]
    fn from(i: usize) -> Self {
        LayoutId {
            inner: i
        }
    }
}

impl Index<ClientId> for WindowTree {
    type Output = Client;

    fn index(&self, index: ClientId) -> &Self::Output {
        match self.tree.get(index.inner).value {
            Window::Client(ref client) => client,
            _ => panic!("WindowTree: invalid client id"),
        }
    }
}

impl IndexMut<ClientId> for WindowTree {
    fn index_mut(&mut self, index: ClientId) -> &mut Self::Output {
        match self.tree.get_mut(index.inner).value {
            Window::Client(ref mut client) => client,
            _ => panic!("WindowTree: invalid client id"),
        }
    }
}


impl Index<LayoutId> for WindowTree {
    type Output = dyn Layout;

    fn index(&self, index: LayoutId) -> &Self::Output {
        match self.tree.get(index.inner).value {
            Window::Layout(ref layout) => layout.as_ref(),
            _ => panic!("WindowTree: invalid layout id"),
        }
    }
}

impl IndexMut<LayoutId> for WindowTree {
    fn index_mut(&mut self, index: LayoutId) -> &mut Self::Output {
        match self.tree.get_mut(index.inner).value {
            Window::Layout(ref mut layout) => layout.as_mut(),
            _ => panic!("WindowTree: invalid layout id"),
        }
    }
}
