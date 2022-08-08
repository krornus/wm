use crate::rect::{Rect, Contains};
use crate::client::Client;

use xcb::x;

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
}

pub struct Container {
    pub rect: Rect,
    focus: Option<usize>,
    children: Vec<ContainerNode>,
}

impl Container {
    pub fn new(rect: Rect) -> Self {
        Container {
            rect: rect,
            focus: None,
            children: vec![],
        }
    }

    pub fn scope(&mut self, rect: Rect) {
       assert!(self.rect.contains(&rect));
        let con = ContainerNode::Container(Self::new(rect));
        self.children.push(con);
    }

    pub fn client(&mut self, client: Client) {
        let con = ContainerNode::Client(client);
        self.children.push(con);
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
                    if c.window == window {
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

    pub fn merge(&mut self, other: Container) {
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
                    if c.window == window {
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
