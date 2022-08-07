use std::collections::VecDeque;

use crate::rect::{Rect, Contains};
use crate::client::Client;

use xcb::x;

pub struct ContainerIterator<'a> {
    containers: Vec<&'a Container>,
}

impl<'a> Iterator for ContainerIterator<'a> {
    type Item = &'a Container;

    fn next(&mut self) -> Option<Self::Item> {
        let con = self.containers.pop()?;

        self.containers.reserve(con.children.len());
        for child in con.children.iter() {
            self.containers.push(child);
        }

        Some(con)
    }
}

pub struct ClientIterator<'a> {
    clients: std::slice::Iter<'a, Client>,
    containers: ContainerIterator<'a>,
}

impl<'a> Iterator for ClientIterator<'a> {
    type Item = &'a Client;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.clients.next() {
            return Some(x);
        }

        let con = self.containers.next()?;
        self.clients = con.stack.iter();
        self.next()
    }
}

pub struct Container {
    rect: Rect,
    stack: Vec<Client>,
    active: Option<usize>,
    children: Vec<Container>,
}

impl Container {
    pub fn new(rect: Rect, stack: Vec<Client>) -> Self {
        Container {
            rect: rect,
            stack: stack,
            active: None,
            children: vec![],
        }
    }

    pub fn add(&mut self, con: Container) {
        assert!(self.rect.contains(&con.rect));
        self.children.push(con)
    }

    /// get an iterator of all containers
    pub fn containers(&self) -> ContainerIterator<'_> {
        ContainerIterator {
            containers: vec![self]
        }
    }

    /// get an iterator of all clients
    pub fn clients(&self) -> ClientIterator<'_> {
        let mut containers = self.containers();
        containers.next().unwrap();

        ClientIterator {
            clients: self.stack.iter(),
            containers: containers,
        }
    }

    /// get a mutable reference to a client based on its window
    pub fn client(&self, window: x::Window) -> Option<&Client> {
        self.clients().find(|c| &c.window == &window)
    }
}
