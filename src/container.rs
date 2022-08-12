// use crate::tree;
// use crate::rect::Rect;
// use crate::client::Client;

// pub type ContainerId = tree::TreeIndex;

// pub struct Scope {
//     pub rect: Rect,
//     pub clients: Vec<Client>,
// }

// impl Scope {
//     pub fn new(rect: Rect) -> Self {
//         Scope {
//             rect: rect,
//             clients: vec![],
//         }
//     }
// }

// pub struct Container {
//     tree: tree::Tree<Scope>,
// }

// impl Container {
//     pub fn new(rect: Rect) -> Self {
//         Container {
//             tree: tree::Tree::new(Scope::new(rect))
//         }
//     }

//     #[inline]
//     pub fn root(&self) -> ContainerId {
//         self.tree.root()
//     }

//     #[inline]
//     pub fn insert(&mut self, id: &ContainerId, value: Scope) -> Option<ContainerId> {
//         self.tree.insert(id, value)
//     }

//     #[inline]
//     pub fn discard(&mut self, id: &ContainerId) {
//         self.tree.discard(id)
//     }

//     #[inline]
//     pub fn get(&self, id: &ContainerId) -> Option<&Scope> {
//         self.tree.get(id).map(|n| &n.value)
//     }

//     #[inline]
//     pub fn get_mut(&mut self, id: &ContainerId) -> Option<&mut Scope> {
//         self.tree.get_mut(id).map(|n| &mut n.value)
//     }
// }

// impl Container {
//     #[inline]
//     pub fn iter<'a>(&'a mut self, id: &'a ContainerId) -> tree::Iter<'a, Scope> {
//         self.tree.iter(id)
//     }

//     #[inline]
//     pub fn children<'a>(&'a mut self, id: &'a ContainerId) -> tree::Children<'a, Scope> {
//         self.tree.children(id)
//     }
// }
