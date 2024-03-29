use std::fmt;

use slab;

pub struct TreeNode<T> {
    pub value: T,
    index: usize,
    parent: Option<usize>,
    first: Option<usize>,
    last: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl<T> fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeNode<T>")
         .field("index", &self.index)
         .field("parent", &self.parent)
         .field("first", &self.first)
         .field("last", &self.last)
         .field("left", &self.left)
         .field("right", &self.right)
         .finish()
    }
}

impl<T> TreeNode<T> {
    fn new(index: usize, value: T) -> Self {
        TreeNode {
            value: value,
            index: index,
            parent: None,
            left: None,
            right: None,
            first: None,
            last: None,
        }
    }
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    #[inline]
    pub fn previous_sibling(&self) -> Option<usize> {
        self.left
    }

    #[inline]
    pub fn next_sibling(&self) -> Option<usize> {
        self.right
    }

    #[inline]
    pub fn first_child(&self) -> Option<usize> {
        self.first
    }

    #[inline]
    pub fn last_child(&self) -> Option<usize> {
        self.last
    }
}

pub struct Tree<T> {
    root: Option<usize>,
    slab: slab::Slab<TreeNode<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        let slab = slab::Slab::new();

        Tree {
            root: None,
            slab: slab,
        }
    }

    pub fn set_root(&mut self, index: Option<usize>) {
        self.root = index;
    }

    pub fn swap_root(&mut self, value: T) -> Option<T> {
        let index = self.slab.vacant_key();
        let node = TreeNode::new(index, value);
        self.slab.insert(node);

        let swap = match self.root {
            Some(i) => Some(self.slab.remove(i).value),
            None => None,
        };

        self.root = Some(index);

        swap
    }

    pub fn root(&self) -> Option<usize> {
        self.root
    }

    pub fn orphan(&mut self, value: T) -> usize {
        let insert_index = self.slab.vacant_key();
        let node = TreeNode::new(insert_index, value);
        self.slab.insert(node)
    }

    pub fn adopt(&mut self, index: usize, child: usize) {
        /* set the parent index in the new child */
        let parent = self.get_mut(index);
        let sibling = parent.last.replace(child);

        /* replace the index of the child with the new key,
         * and set the child's sibing to the new key. */
        match sibling {
            Some(i) => {
                self.get_mut(i).right = Some(child);
            }
            _ => {
                /* no children - set first and last */
                parent.first = Some(child);
            }
        }

        let mut node = self.get_mut(child);
        node.parent = Some(index);
        node.left = sibling;

    }

    pub fn insert(&mut self, index: usize, value: T) -> usize {
        let child = self.orphan(value);
        self.adopt(index, child);

        child
    }

    #[inline]
    pub fn get(&self, index: usize) -> &TreeNode<T> {
        self.slab.get(index).expect("index out of bounds")
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> &mut TreeNode<T> {
        self.slab.get_mut(index).expect(&format!("index out of bounds: {}", index))
    }

    /// Remove a sub-tree from one tree and graft it into another
    pub fn graft(&mut self, other: &mut Tree<T>, from: usize, to: usize) {
        /* not the fastest way to do this, but the easiest to read */
        let children: Vec<_> = other.children(from).collect();
        let node = other.slab.remove(from);
        let index = self.insert(to, node.value);

        for child in children.into_iter().rev() {
            self.graft(other, child, index);
        }
    }

    /// Discard a node and its children from a tree with no further processing
    fn discard(&mut self, index: usize) -> TreeNode<T> {
        let children: Vec<_> = self.children(index).collect();

        let root = self.slab.remove(index);

        for child in children.into_iter() {
            self.discard(child);
        }

        root
    }

    /// Extract a node from the tree and return its value, non-recursively
    pub fn extract(&mut self, index: usize) -> T {
        let root = self.slab.remove(index);

        /* check the left sibling. if there isn't one, we are the first child in
         * parent. otherwise, update the right sibling. */
        match root.left {
            Some(i) => {
                /* we have a sibling to the left. give it our right value */
                self.get_mut(i).right = root.right;
            },
            None => {
                /* we are the first child for our parent.
                 * update parent.first to have our right value */
                match root.parent {
                    Some(j) => {
                        self.get_mut(j).first = root.right;
                    },
                    None => {
                    }
                }
            },
        }

        /* check the right sibling. if there isn't one, we are the last child in
         * parent. otherwise, update the left sibling. */
        match root.right {
            Some(i) => {
                /* we have a sibling to the right. give it our left value */
                self.get_mut(i).left = root.left;
            },
            None => {
                /* we are the last child for our parent.
                 * update parent.last to have our left value */
                match root.parent {
                    Some(j) => {
                        self.get_mut(j).last = root.left;
                    },
                    None => {
                    }
                }
            },
        }

        root.value
    }

    /// Extract a node from the tree and return its value, recursively discarding children
    pub fn prune(&mut self, index: usize) -> T {
        let children: Vec<_> = self.children(index).collect();

        for child in children.into_iter() {
            self.discard(child);
        }

        self.extract(index)
    }

    /// Remove a node from the tree, returning a new tree with root at node
    pub fn remove(&mut self, index: usize) -> Tree<T> {
        let children: Vec<_> = self.children(index).collect();

        let root = self.slab.remove(index);
        let mut tree = Tree::new();
        tree.swap_root(root.value);

        let parent = tree.root.unwrap();

        for child in children.into_iter().rev() {
            tree.graft(self, child, parent);
        }

        tree
    }
}

impl<T> Tree<T> {
    pub fn children<'a>(&'a self, index: usize) -> Children<'a, T> {
        let child = self.get(index).first;

        Children {
            tree: self,
            index: child,
        }
    }

    pub fn iter_at<'a>(&'a self, index: usize) -> IterAt<'a, T> {
        IterAt {
            tree: self,
            stack: vec![index],
        }
    }

    pub fn iter(&self) -> slab::Iter<'_, TreeNode<T>> {
        self.slab.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> slab::IterMut<'a, TreeNode<T>> {
        self.slab.iter_mut()
    }
}

pub struct Children<'a, T> {
    tree: &'a Tree<T>,
    index: Option<usize>,
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let node = self.tree.get(index);
        self.index = node.next_sibling();

        Some(index)
    }
}

pub struct IterAt<'a, T> {
    tree: &'a Tree<T>,
    stack: Vec<usize>,
}

impl<'a, T> Iterator for IterAt<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;

        self.stack.extend(self.tree.children(index));

        Some(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn children<T: Copy>(tree: &Tree<T>, index: usize) -> Vec<T> {
        let i: Vec<_> = tree.children(index).collect();
        i.into_iter().map(|i| tree.get(&i).unwrap().value).collect()
    }

    fn iter<T: Copy>(tree: &Tree<T>, index: usize) -> Vec<T> {
        let i: Vec<_> = tree.iter(index).collect();
        i.into_iter().map(|i| tree.get(&i).unwrap().value).collect()
    }

    #[test]
    fn test_tree() {
        let mut tree = Tree::new(1);

        let two = tree.insert(&tree.root(), 2).unwrap();
        tree.insert(&two, 3).unwrap();
        let four = tree.insert(&two, 4).unwrap();

        tree.insert(&four, 5).unwrap();
        tree.insert(&four, 6).unwrap();
        tree.insert(&four, 7).unwrap();

        tree.insert(&tree.root(), 8).unwrap();

        assert_eq!(children(&tree, &tree.root()), vec![8, 2]);
        assert_eq!(children(&tree, &two), vec![4, 3]);
        assert_eq!(children(&tree, &four), vec![7, 6, 5]);
        assert_eq!(iter(&tree, &tree.root), vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let new = tree.remove(&two).unwrap();
        assert_eq!(iter(&tree, &tree.root), vec![1, 8]);
        assert_eq!(iter(&new, &new.root), vec![2, 3, 4, 5, 6, 7]);
    }
}
