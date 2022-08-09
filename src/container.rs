use std::sync::atomic::{AtomicU64, Ordering};

use crate::rect::Rect;
use crate::client::Client;
use crate::error::Error;

use xcb::x;

pub type ContainerID = u64;

fn id() -> ContainerID {
    static ID: AtomicU64 = AtomicU64::new(0);

    let id = ID.load(Ordering::Relaxed);

    /* hopefully you never need this many */
    if id == ContainerID::MAX {
        panic!("maximum number of containers reached");
    }

    ID.store(id + 1, Ordering::Relaxed);

    id
}

pub enum ContainerNode {
    Container(Container),
    Client(Client),
}

impl ContainerNode {
    fn into_container(self) -> Container {
        match self {
            ContainerNode::Container(c) => c,
            ContainerNode::Client(_) => panic!("into_container() on Client variant"),
        }
    }

    fn into_client(self) -> Client {
        match self {
            ContainerNode::Client(c) => c,
            ContainerNode::Container(_) => panic!("into_client() on Container variant"),
        }
    }

    fn as_container(&self) -> &Container {
        match self {
            ContainerNode::Container(c) => c,
            ContainerNode::Client(_) => panic!("as_container() on Client variant"),
        }
    }

    fn as_client(&self) -> &Client {
        match self {
            ContainerNode::Client(c) => c,
            ContainerNode::Container(_) => panic!("as_client() on Container variant"),
        }
    }

    fn as_container_mut(&mut self) -> &mut Container {
        match self {
            ContainerNode::Container(ref mut c) => c,
            ContainerNode::Client(_) => panic!("as_container_mut() on Client variant"),
        }
    }

    fn as_client_mut(&mut self) -> &mut Client {
        match self {
            ContainerNode::Client(ref mut c) => c,
            ContainerNode::Container(_) => panic!("as_client_mut() on Container variant"),
        }
    }
}

pub struct Container {
    id: ContainerID,
    rect: Rect,
    children: Vec<ContainerNode>,
    focus: Option<usize>,
}

impl Container {
    pub fn new(rect: Rect) -> Self {
        Container {
            rect: rect,
            id: id(),
            focus: None,
            children: vec![],
        }
    }

    #[inline]
    pub fn id(&self) -> ContainerID {
        self.id
    }

    #[inline]
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn scope(&mut self, rect: Rect) -> &mut Container {
        let con = ContainerNode::Container(Self::new(rect));
        let idx = self.children.len();
        self.children.push(con);

        match &mut self.children[idx] {
            ContainerNode::Container(ref mut c) => c,
            _ => unreachable!(),
        }
    }

    fn insert(&mut self, window: x::Window, rect: Rect) -> &mut Client {
        let client = Client::new(window, rect);
        let con = ContainerNode::Client(client);
        let idx = self.children.len();
        self.children.push(con);

        match &mut self.children[idx] {
            ContainerNode::Client(ref mut c) => c,
            _ => unreachable!(),
        }
    }

    pub fn add(&mut self, id: ContainerID, window: x::Window, rect: Rect) -> Result<&mut Client, Error> {
        for con in self.children.iter_mut() {
            match con {
                ContainerNode::Container(c) => {
                    if c.id == id {
                        return Ok(c.insert(window, rect));
                    } else {
                        let client = c.add(id, window, rect);
                        if client.is_ok() {
                            return client;
                        }
                    }
                },
                _ => { },
            }
        }

        Err(Error::ContainerNotFound)
    }

    pub fn by_window(&self, window: x::Window) -> Option<&Container> {
        for con in self.children.iter() {
            match con {
                ContainerNode::Client(c) => {
                    if c.window() == window {
                        return Some(self);
                    }
                },
                ContainerNode::Container(c) => {
                    let child = c.by_window(window);
                    if child.is_some() {
                        return child;
                    }
                },
            }
        }

        None
    }

    #[inline]
    pub fn focus(&mut self, window: x::Window) {
        self.focus_client(window);
    }

    pub fn remove(&mut self, window: x::Window) -> Option<Client> {
        let mut index = None;

        for (i, con) in self.children.iter_mut().enumerate() {
            match con {
                ContainerNode::Client(c) => {
                    if c.window() == window {
                        index = Some(i);
                        break;
                    }
                },
                ContainerNode::Container(s) => {
                    let client = s.remove(window);
                    if client.is_some() {
                        return client;
                    }
                },
            }
        }

        if let Some(i) = index {
            let con = self.children.remove(i);
            Some(con.into_client())
        } else {
            None
        }
    }

    pub fn merge(&mut self, _: Container) {
        /* TODO: for when monitor is disconnected */
    }

    pub fn clients_mut(&mut self, recursive: bool) -> Vec<&mut Client> {
        /* this is not an iterator on purpose. it is used for layouts,
         * which must know the total number of clients before
         * the rendering process begins. */
        let mut clients = Vec::with_capacity(self.children.len());

        for x in self.children.iter_mut() {
            match x {
                ContainerNode::Container(c) => {
                    if recursive {
                        let mut refs = c.clients_mut(recursive);
                        clients.append(&mut refs);
                    }
                },
                ContainerNode::Client(c) => {
                    clients.push(c);
                }
            }
        }

        clients
    }
}

impl Container {
    fn focus_client(&mut self, window: x::Window) -> bool {
        let mut focus = None;

        /* dont break loop early, we need to reset any previously focused items to false */
        for (i, con) in self.children.iter_mut().enumerate() {
            match con {
                ContainerNode::Client(c) => {
                    if c.window() == window {
                        focus = Some(i);
                        c.focus(true);
                    } else {
                        c.focus(false);
                    }
                },
                ContainerNode::Container(s) => {
                    if s.focus_client(window) {
                        focus = Some(i);
                    }
                },
            }
        }

        if focus.is_some() {
            self.focus = focus;
            true
        } else {
            self.focus = None;
            false
        }
    }
}
