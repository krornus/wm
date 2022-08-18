use std::collections::HashMap;

use crate::slab::{SlabIndex, Slab};

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
            tags: tags.iter().map(|x| String::from(x.as_ref())).collect()
        }
    }
}

pub struct TagMask {
    mask: BitVec,
}

impl TagMask {
    pub fn new() -> Self {
        TagMask {
            mask: bitvec![1],
        }
    }

    pub fn set(&mut self, tag: Tag) {
        match tag {
            Tag::On(i) => self.mask.set(i, true),
            Tag::Off(i) => self.mask.set(i, false),
            Tag::Toggle(i) => {
                let mut p = self.mask.get_mut(i)
                    .expect("tag index out of range");
                *p = !*p;
            },
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
    tagmasks: HashMap<TagSetId, TagMask>
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

    pub fn mask_mut(&mut self, id: TagSetId) -> Option<&mut TagMask> {
        self.tagmasks.get_mut(&id)
    }

    pub fn visible(&self, id: TagSetId, selection: &TagMask) -> Option<bool> {
        self.tagmasks.get(&id).map(|mask| {
            mask.visible(selection)
        })
    }
}
