use std::collections::HashMap;

use crate::slab::{Slab, SlabIndex};

use bitvec::prelude::*;

pub type TagSetId = SlabIndex;

#[derive(Debug, Copy, Clone)]
pub enum Tag {
    On(usize),
    Off(usize),
    Toggle(usize),
}

pub struct TagSet {
    tags: Vec<String>,
}

impl TagSet {
    pub fn new<T: AsRef<str>>(tags: Vec<T>) -> Self {
        TagSet {
            tags: tags.iter().map(|x| String::from(x.as_ref())).collect(),
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

    fn set_at(&mut self, i: usize, p: bool) {
        self.mask.resize(i + 1, false);
        self.mask.set(i, p);
    }

    pub fn set(&mut self, tag: Tag) {
        match tag {
            Tag::On(i) => self.set_at(i, true),
            Tag::Off(i) => self.set_at(i, false),
            Tag::Toggle(i) => {
                self.mask.resize(i + 1, false);
                let mut p = self.mask.get_mut(i).expect("tag index out of range");
                *p = !*p;
            }
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.mask.clear()
    }

    #[inline]
    pub fn visible(&self, other: &TagMask) -> bool {
        /* lazy - should be fine. tagsets shouldn't get too big */
        (self.mask.clone() & other.mask.clone()).any()
    }
}

pub struct Tags {
    tagsets: Slab<TagSet>,
    tagmasks: HashMap<TagSetId, TagMask>,
}

impl Tags {
    pub fn new() -> Self {
        Tags {
            tagsets: Slab::new(),
            tagmasks: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tagset: TagSet) -> TagSetId {
        let id = self.tagsets.insert(tagset);
        self.tagmasks.insert(id, TagMask::new());

        id
    }

    pub fn mask(&self, id: TagSetId) -> Option<&TagMask> {
        self.tagmasks.get(&id)
    }

    pub fn mask_mut(&mut self, id: TagSetId) -> Option<&mut TagMask> {
        self.tagmasks.get_mut(&id)
    }

    pub fn masks(&self) -> &HashMap<TagSetId, TagMask> {
        &self.tagmasks
    }

    pub fn visible(&self, id: TagSetId, selection: &TagMask) -> Option<bool> {
        self.tagmasks.get(&id).map(|mask| mask.visible(selection))
    }

    pub fn select<'a, 'b>(&'a self, ids: &'b [TagSetId]) -> TagSelection<'a, 'b> {
        TagSelection {
            tags: self,
            selection: ids,
        }
    }
}

pub struct TagSelection<'a, 'b> {
    tags: &'a Tags,
    selection: &'b [TagSetId],
}

impl<'a, 'b> TagSelection<'a, 'b> {
    pub fn iter(&self) -> impl Iterator<Item = (TagSetId, &TagMask)> {
        self.selection
            .iter()
            .map(move |x| (*x, self.tags.mask(*x)))
            .filter_map(|(x, y)| y.and_then(|mask| Some((x, mask))))
    }
}
