/// Inspired by crate tree_slab, which is no longer being updated.
use slab::Slab;

/// Generational index into the slab
#[derive(Debug, Clone)]
struct TreeIndex {
    key: usize,
    generation: usize,
}

struct Node<T> {
    pub value: T,
    generation: usize,
    parent: Option<TreeIndex>,
    left: Option<TreeIndex>,
    right: Option<TreeIndex>,
    child: Option<TreeIndex>,
}

impl<T> Node<T> {
    fn new(value: T, generation: usize) -> Self {
        Node {
            value: value,
            generation: generation,
            parent: None,
            left: None,
            right: None,
            child: None,
        }
    }
}

struct Tree<T> {
    root: TreeIndex,
    slab: Slab<Node<T>>,
    generation: usize,
}

impl<T> Tree<T> {
    fn new(value: T) -> Self {
        let mut slab = Slab::new();
        let node = Node::new(value, 0);
        let key = TreeIndex { key: slab.insert(node), generation: 0 };

        Tree {
            root: key,
            slab: slab,
            generation: 0,
        }
    }

    fn node(&self, value: T) -> Node<T> {
        Node::new(value, self.generation)
    }

    fn index(&self) -> TreeIndex {
        TreeIndex { key: self.slab.vacant_key(), generation: self.generation }
    }

    pub fn root(&mut self, value: T) -> TreeIndex {
        let node = self.node(value);
        let key = self.slab.insert(node);

        TreeIndex { key: key, generation: self.generation }
    }

    pub fn insert(&mut self, index: &TreeIndex, value: T) -> Option<TreeIndex> {
        let insert_index = self.index();
        let mut node = self.node(value);

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

        let key = self.slab.insert(node);
        Some(TreeIndex { key: key, generation: self.generation })
    }

    pub fn get(&self, index: &TreeIndex) -> Option<&Node<T>> {
        self.slab.get(index.key).and_then(|node|
            if index.generation == node.generation {
                Some(node)
            } else {
                None
            })
    }

    pub fn get_mut(&mut self, index: &TreeIndex) -> Option<&mut Node<T>> {
        self.slab.get_mut(index.key).and_then(|node|
            if index.generation == node.generation {
                Some(node)
            } else {
                None
            })
    }

    pub fn remove(&mut self, index: &TreeIndex) -> Option<T> {
        let generation = self.slab.get(index.key)
            .map(|node| node.generation);

        if generation == Some(index.generation) {
            self.generation += 1;
            let node = self.slab.remove(index.key);
            Some(node.value)
        } else {
            None
        }
    }
}

impl<T> Tree<T> {
    fn children<'a>(&'a self, index: &'a TreeIndex) -> Children<'a, T> {
        Children {
            tree: self,
            index: Some(index),
        }
    }

    fn iter<'a>(&'a self, index: &'a TreeIndex) -> Iter<'a, T> {
        Iter {
            tree: self,
            stack: vec![index]
        }
    }
}

struct Children<'a, T> {
    tree: &'a Tree<T>,
    index: Option<&'a TreeIndex>,
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = &'a TreeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let node = self.tree.get(index)?;
        self.index = node.right.as_ref();

        Some(index)
    }
}

struct Iter<'a, T> {
    tree: &'a Tree<T>,
    stack: Vec<&'a TreeIndex>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a TreeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;

        self.stack.extend(self.tree.children(index));

        Some(index)
    }
}
