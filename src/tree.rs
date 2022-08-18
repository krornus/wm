use slab;

pub struct TreeNode<T> {
    pub value: T,
    index: usize,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    child: Option<usize>,
}

impl<T> TreeNode<T> {
    fn new(index: usize, value: T) -> Self {
        TreeNode {
            value: value,
            index: index,
            parent: None,
            left: None,
            right: None,
            child: None,
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
    pub fn child(&self) -> Option<usize> {
        self.child
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
        let child_index = parent.child.replace(child);

        /* replace the index of the child with the new key,
         * and set the child's sibing to the new key. */
        match child_index {
            Some(i) => {
                let child = self.get_mut(i);
                child.left = Some(index);
            },
            _ => {
            }
        }

        let mut node = self.get_mut(child);
        node.parent = Some(index);
        node.right = child_index;
    }

    pub fn insert(&mut self, index: usize, value: T) -> usize {
        let child = self.orphan(value);
        self.adopt(index, child);
        child
    }

    #[inline]
    pub fn get(&self, index: usize) -> &TreeNode<T> {
        self.slab.get(index)
            .expect("index out of bounds")
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> &mut TreeNode<T> {
        self.slab.get_mut(index)
            .expect("index out of bounds")
    }

    /// Take a sub-tree from one tree and place it in another
    pub fn take(&mut self, other: &mut Tree<T>, from: usize, to: usize) {
        /* not the fastest way to do this, but the easiest to read */
        let children: Vec<_> = other.children(from).collect();
        let node = other.slab.remove(from);
        let index = self.insert(to, node.value);

        for child in children.into_iter().rev() {
            self.take(other, child, index);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Tree<T>> {
        let children: Vec<_> = self.children(index).collect();

        let root = self.slab.try_remove(index)?;
        let mut tree = Tree::new();
        tree.swap_root(root.value);

        let parent = tree.root.unwrap();

        for child in children.into_iter().rev() {
            tree.take(self, child, parent);
        }

        Some(tree)
    }
}

impl<T> Tree<T> {
    pub fn children<'a>(&'a self, index: usize) -> Children<'a, T> {
        let child = self.get(index).child;

        Children {
            tree: self,
            index: child,
        }
    }

    pub fn iter_at<'a>(&'a self, index: usize) -> IterAt<'a, T> {
        IterAt {
            tree: self,
            stack: vec![index]
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
