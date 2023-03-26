use crate::tree::Tree;
use crate::rect::Rect;
use crate::client::{Client, ClientId};
use crate::layout::{Cell, Layout};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct PaneId {
    id: usize,
}

pub struct PaneTree {
    tree: Tree<Pane>,
}

struct MaskTree {
    tree: Tree<usize>,
}

pub struct Pane {
    layout: Box<dyn Layout>,
    focus: ClientId,
    visible: bool,
    size: Rect,
}

pub enum Cursor {
    Client(Client),
    Pane(Pane),
}

impl PaneTree {
    pub fn new() -> Self {
        PaneTree {
            tree: Tree::new(),
        }
    }
}
