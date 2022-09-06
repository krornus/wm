use std::ops::{Index, IndexMut};

pub trait AsIndex {
    fn as_index(&self) -> usize;
}

pub struct SlabMap<T> {
    map: Vec<Option<T>>,
}

pub struct Iter<'a, T> {
    index: usize,
    slab: &'a SlabMap<T>,
}

impl<T: Clone> SlabMap<T> {
    pub fn new() -> Self {
        SlabMap {
            map: Vec::new()
        }
    }

    pub fn insert(&mut self, key: usize, value: T) -> Option<T> {
        if key == self.map.len() {
            /* fast expand */
            self.map.push(Some(value));
            None
        } else if key > self.map.len() {
            /* need to expand the vector for this key */
            self.map.resize(key + 1, None);
            self.map[key] = Some(value);
            None
        } else {
            /* swap value -- may be none */
            std::mem::replace(&mut self.map[key], Some(value))
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if key < self.map.len() {
            std::mem::replace(&mut self.map[key], None)
        } else {
            None
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        if key < self.map.len() {
            self.map[key].as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        if key < self.map.len() {
            self.map[key].as_mut()
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            index: 0,
            slab: self,
        }
    }
}

impl<T> Index<usize> for SlabMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.map[index].as_ref()
            .expect(&format!("index out of bounds: {}", index))
    }
}

impl<T> IndexMut<usize> for SlabMap<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.map[index].as_mut()
            .expect(&format!("index out of bounds: {}", index))
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.slab.map.len() {
            let i = self.index;
            let o = &self.slab.map[self.index];

            self.index += 1;

            match o {
                Some(ref v) => {
                    return Some((i, v));
                },
                None => {
                },
            }

        }

        None
    }
}
