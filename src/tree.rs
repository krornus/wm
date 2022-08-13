/// Inspired by crate tree_slab, which is no longer being updated.
use crate::slab::{self, Slab, SlabIndex};

pub struct TreeNode<T> {
    pub value: T,
    parent: Option<SlabIndex>,
    left: Option<SlabIndex>,
    right: Option<SlabIndex>,
    child: Option<SlabIndex>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value: value,
            parent: None,
            left: None,
            right: None,
            child: None,
        }
    }
}

pub struct Tree<T> {
    root: SlabIndex,
    slab: Slab<TreeNode<T>>,
}

impl<T> Tree<T> {
    pub fn new(value: T) -> Self {
        let mut slab = Slab::new();

        let node = TreeNode::new(value);
        let index = slab.insert(node);

        Tree {
            root: index,
            slab: slab,
        }
    }

    pub fn root(&self) -> SlabIndex {
        self.root
    }

    pub fn insert(&mut self, index: &SlabIndex, value: T) -> Option<SlabIndex> {
        let insert_index = self.slab.vacant_key();
        let mut node = TreeNode::new(value);

        /* set the parent index in the new child */
        let parent = self.get_mut(index)?;
        node.parent = Some(index.clone());

        let child_index = parent.child.replace(insert_index);

        /* replace the index of the child with the new key,
         * and set the child's sibing to the new key. */
        match child_index {
            Some(ref i) => {
                let child = self.get_mut(i)
                                .expect("invalid child index");
                child.left = Some(index.clone());
            },
            _ => {
            }
        }

        node.right = child_index;

        Some(self.slab.insert(node))
    }

    #[inline]
    pub fn get(&self, index: &SlabIndex) -> Option<&TreeNode<T>> {
        self.slab.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: &SlabIndex) -> Option<&mut TreeNode<T>> {
        self.slab.get_mut(index)
    }

    /// Take a sub-tree from one tree and place it in another
    pub fn take(&mut self, other: &mut Tree<T>, from: &SlabIndex, to: &SlabIndex) {
        /* not the fastest way to do this, but the easiest to read */
        let children: Vec<_> = other.children(*from).collect();
        let node = other.slab.remove(from);
        let index = self.insert(to, node.value)
            .expect("corrupted tree");

        for child in children.into_iter().rev() {
            self.take(other, &child, &index);
        }
    }

    pub fn remove(&mut self, index: &SlabIndex) -> Option<Tree<T>> {
        let children: Vec<_> = self.children(*index).collect();

        let root = self.slab.try_remove(index)?;
        let mut tree = Tree::new(root.value);

        let parent = tree.root;

        for child in children.into_iter().rev() {
            tree.take(self, &child, &parent);
        }

        Some(tree)
    }
}

impl<T> Tree<T> {
    pub fn children<'a>(&'a self, index: SlabIndex) -> Children<'a, T> {
        let child = self.get(&index).and_then(|x| x.child);

        Children {
            tree: self,
            index: child,
        }
    }

    pub fn iter_at<'a>(&'a self, index: SlabIndex) -> IterAt<'a, T> {
        match self.get(&index) {
            Some(_) => IterAt {
                tree: self,
                stack: vec![index]
            },
            None => IterAt {
                tree: self,
                stack: vec![],
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.slab.iter()
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            iter: self.slab.iter_mut()
        }
    }
}

pub struct Children<'a, T> {
    tree: &'a Tree<T>,
    index: Option<SlabIndex>,
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = SlabIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let node = self.tree.get(&index)?;
        self.index = node.right;

        Some(index)
    }
}

pub struct IterAt<'a, T> {
    tree: &'a Tree<T>,
    stack: Vec<SlabIndex>,
}

impl<'a, T> Iterator for IterAt<'a, T> {
    type Item = SlabIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;

        self.stack.extend(self.tree.children(index));

        Some(index)
    }
}

pub struct Iter<'a, T> {
    iter: slab::Iter<'a, TreeNode<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, v) = self.iter.next()?;
        Some(&v.value)
    }
}

pub struct IterMut<'a, T> {
    iter: slab::IterMut<'a, TreeNode<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, v) = self.iter.next()?;
        Some(&mut v.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn children<T: Copy>(tree: &Tree<T>, index: &SlabIndex) -> Vec<T> {
        let i: Vec<_> = tree.children(index).collect();
        i.into_iter().map(|i| tree.get(&i).unwrap().value).collect()
    }

    fn iter<T: Copy>(tree: &Tree<T>, index: &SlabIndex) -> Vec<T> {
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
