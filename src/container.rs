use crate::tree;
use crate::wm::Adapter;
use crate::rect::Rect;
use crate::client::Client;
use crate::layout::Layout;
use crate::slab::SlabIndex;

use xcb::x;

pub type ContainerId = SlabIndex;

pub struct Scope {
    rect: Rect,
    layout: Box<dyn Layout>,
    clients: Vec<Client>,
}

impl Scope {
    pub fn new(rect: Rect, layout: impl Layout + 'static) -> Self {
        Scope {
            rect: rect,
            layout: Box::new(layout),
            clients: vec![],
        }
    }

    #[inline]
    fn take(&mut self, mut other: Self) {
        self.clients.append(&mut other.clients);
    }

    #[inline]
    pub fn clients(&self) -> &[Client] {
        &self.clients
    }

    #[inline]
    pub fn add_client(&mut self, client: Client) -> &mut Client {
        let index = self.clients.len();
        self.clients.push(client);
        &mut self.clients[index]
    }

    #[inline]
    pub fn get_client(&self, window: x::Window) -> Option<&Client> {
        self.clients.iter().find(|x| x.window() == window)
    }

    #[inline]
    pub fn get_client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.clients.iter_mut().find(|x| x.window() == window)
    }
}

pub struct Container {
    tree: tree::Tree<Scope>,
}

impl Container {
    pub fn new(rect: Rect, layout: impl Layout + 'static) -> Self {
        Container {
            tree: tree::Tree::new(Scope::new(rect, layout))
        }
    }

    #[inline]
    pub fn root(&self) -> ContainerId {
        self.tree.root()
    }

    #[inline]
    pub fn add_client(&mut self, id: &ContainerId, client: Client) -> &mut Client {
        let scope = self.get_mut(id).unwrap();
        scope.add_client(client)
    }

    #[inline]
    pub fn get_client(&self, window: x::Window) -> Option<&Client> {
        self.tree.iter().find_map(|scope| scope.get_client(window))
    }

    #[inline]
    pub fn get_client_mut(&mut self, window: x::Window) -> Option<&mut Client> {
        self.tree.iter_mut().find_map(|scope| scope.get_client_mut(window))
    }

    #[inline]
    pub fn insert(&mut self, id: &ContainerId, value: Scope) -> Option<ContainerId> {
        self.tree.insert(id, value)
    }

    pub fn take(&mut self, mut other: Container) {
        let root = other.tree.root();
        let children: Vec<_> = other.tree.children(root)
            .collect();

        let parent = self.tree.root();

        for child in children.into_iter().rev() {
            self.tree.take(&mut other.tree, &child, &parent);
        }
    }

    pub fn arrange(&mut self, adapter: &mut Adapter) {
        for scope in self.tree.iter_mut() {
            scope.layout.arrange(adapter, &scope.rect, &mut scope.clients);
        }
    }

    #[inline]
    pub fn get(&self, id: &ContainerId) -> Option<&Scope> {
        self.tree.get(id).map(|n| &n.value)
    }

    #[inline]
    pub fn get_mut(&mut self, id: &ContainerId) -> Option<&mut Scope> {
        self.tree.get_mut(id).map(|n| &mut n.value)
    }
}
