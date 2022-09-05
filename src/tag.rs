use std::ops::{Index, IndexMut};

use slab::{self, Slab};

use bitvec::prelude::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct TagSetId {
    inner: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Tag {
    On(usize),
    Off(usize),
    Toggle(usize),
}

#[derive(Debug)]
pub struct TagSet {
    names: Vec<String>,
    mask: TagMask,
}

impl TagSet {
    pub fn new<T: AsRef<str>>(names: &[T]) -> Self {
        let names: Vec<String> = names.iter()
            .map(|x| String::from(x.as_ref()))
            .collect();

        let mut mask = bitvec![0; names.len()];
        mask.set(0, true);

        TagSet {
            names: names,
            mask: TagMask::from(mask),
        }
    }

    pub fn tags<'a>(&'a self) -> TagSetValues<'a> {
        TagSetValues {
            index: 0,
            tagset: self,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    #[inline]
    pub fn names(&self) -> &[String] {
        &self.names
    }

    #[inline]
    pub fn mask(&self) -> &TagMask {
        &self.mask
    }

    #[inline]
    pub fn mask_mut(&mut self) -> &mut TagMask {
        &mut self.mask
    }
}

pub struct TagSetValues<'a> {
    index: usize,
    tagset: &'a TagSet,
}

impl<'a> Iterator for TagSetValues<'a> {
    type Item = (&'a str, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.tagset.len() {
            None
        } else {
            let name = &self.tagset.names[self.index];
            let value = self.tagset.mask.get(self.index);
            self.index += 1;

            Some((name, value))
        }
    }
}


#[derive(Debug, Clone)]
pub struct TagMask {
    mask: BitVec,
}

impl TagMask {
    pub fn new() -> Self {
        TagMask { mask: bitvec![1] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.mask.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.mask.clear()
    }

    #[inline]
    pub fn get(&self, index: usize) -> bool {
        *self.mask.get(index).as_deref()
            .unwrap_or(&false)
    }

    /// Resize the mask to support at minimum i values, without shrinking
    fn grow(&mut self, i: usize) {
        if i >= self.mask.len() {
            self.mask.resize(i + 1, false);
        }
    }

    pub fn set(&mut self, tag: Tag) {
        match tag {
            Tag::On(i) => {
                self.grow(i);
                self.mask.set(i, true);
            },
            Tag::Off(i) => {
                self.grow(i);
                self.mask.set(i, false);
            },
            Tag::Toggle(i) => {
                self.grow(i);
                let mut p = self.mask.get_mut(i).unwrap();
                *p = !*p;
            }
        }
    }

    pub fn iter(&self) -> bitvec::slice::BitRefIter<'_, usize, Lsb0> {
        self.mask.iter().by_refs()
    }

    #[inline]
    pub fn visible(&self, other: &TagMask) -> bool {
        /* lazy clone - should be fine. tagsets shouldn't get too big
         * so it should hopefully be similar to a copy */
        (self.mask.clone() & other.mask.clone()).any()
    }
}

impl From<BitVec> for TagMask {
    fn from(bv: BitVec) -> Self {
        TagMask {
            mask: bv,
        }
    }
}

pub struct Tags {
    tagsets: Slab<TagSet>,
}

impl Tags {
    pub fn new() -> Self {
        Tags {
            tagsets: Slab::new(),
        }
    }

    pub fn iter<'a>(&'a self) -> TagSets<'a> {
        TagSets { iter: self.tagsets.iter() }
    }

    pub fn len(&self) -> usize {
        self.tagsets.len()
    }

    pub fn insert(&mut self, tagset: TagSet) -> TagSetId {
        TagSetId { inner: self.tagsets.insert(tagset) }
    }

    pub fn visible(&self, id: TagSetId, selection: &TagMask) -> bool {
        self.tagsets[id.inner].mask.visible(selection)
    }

    pub fn select<'a, 'b>(&'a self, ids: &'b [TagSetId]) -> TagSelection<'a, 'b> {
        TagSelection {
            tags: self,
            selection: ids,
        }
    }
}

pub struct TagSets<'a> {
    iter: slab::Iter<'a, TagSet>
}

impl<'a> Iterator for TagSets<'a> {
    type Item = (TagSetId, &'a TagSet);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .map(|(id, tagset)| {
                (TagSetId { inner: id }, tagset)
            })
    }
}


impl Index<TagSetId> for Tags {
    type Output = TagSet;

    fn index(&self, id: TagSetId) -> &Self::Output {
        &self.tagsets[id.inner]
    }
}

impl IndexMut<TagSetId> for Tags {
    fn index_mut(&mut self, id: TagSetId) -> &mut Self::Output {
        &mut self.tagsets[id.inner]
    }
}

pub struct TagSelection<'a, 'b> {
    tags: &'a Tags,
    selection: &'b [TagSetId],
}

impl<'a, 'b> TagSelection<'a, 'b> {
    pub fn iter(&self) -> impl Iterator<Item = (TagSetId, &TagSet)> {
        self.selection
            .iter()
            .map(move |x| (*x, &self.tags.tagsets[x.inner]))
    }
}
