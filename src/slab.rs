use std::fmt;
use slab;

/// Generational index into the slab
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct SlabIndex {
    key: usize,
    generation: usize,
}

struct SlabValue<T> {
    value: T,
    generation: usize,
}

impl fmt::Display for SlabIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.key, self.generation)
    }
}

pub struct Slab<T> {
    slab: slab::Slab<SlabValue<T>>,
    generation: usize,
}

impl<T> Slab<T> {
    pub fn new() -> Self {
        Slab {
            slab: slab::Slab::new(),
            generation: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> SlabIndex {
        let sv = SlabValue {
            value: value,
            generation: self.generation,
        };

        SlabIndex {
            key: self.slab.insert(sv),
            generation: self.generation,
        }
    }

    pub fn get(&self, index: &SlabIndex) -> Option<&T> {
        self.slab.get(index.key).and_then(|e|
            if &index.generation == &e.generation {
                Some(&e.value)
            } else {
                None
            })
    }

    pub fn get_mut(&mut self, index: &SlabIndex) -> Option<&mut T> {
        self.slab.get_mut(index.key).and_then(|e|
            if &index.generation == &e.generation {
                Some(&mut e.value)
            } else {
                None
            })
    }

    pub fn remove(&mut self, index: &SlabIndex) -> T {
        self.generation += 1;
        self.slab.remove(index.key).value
    }

    pub fn try_remove(&mut self, index: &SlabIndex) -> Option<T> {
        self.get(index)?;
        Some(self.remove(index))
    }

    pub fn vacant_key(&mut self) -> SlabIndex {
        SlabIndex {
            key: self.slab.vacant_key(),
            generation: self.generation,
        }
    }
}
