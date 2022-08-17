use crate::slab::{SlabIndex, Slab};

use bitvec::prelude as bv;

pub type TagSetId = SlabIndex;

type BitVec = bv::BitVec;


pub enum Tag {
}

pub struct TagSet {
    tags: Vec<String>,
}

pub struct Tags {
    slab: Slab<TagSet>,
}

impl Tags {
    pub fn new() -> Self {
        Tags {
            slab: Slab::new(),
        }
    }

    pub fn insert(&mut self, tagset: TagSet) -> TagSetId {
        self.slab.insert(tagset)
    }

}
