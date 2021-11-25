use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
};

pub(crate) trait HEIndex:
    Copy + Clone + Debug + Default + Deref + Hash + Eq + Ord + Send + Sync
{
    fn new(idx: u32) -> Self;
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct VertexIndex {
    idx: u32,
}

impl Deref for VertexIndex {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.idx
    }
}

impl HEIndex for VertexIndex {
    fn new(idx: u32) -> Self {
        VertexIndex { idx }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HalfEdgeIndex {
    idx: u32,
}

impl Deref for HalfEdgeIndex {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.idx
    }
}

impl HEIndex for HalfEdgeIndex {
    fn new(idx: u32) -> Self {
        HalfEdgeIndex { idx }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct FaceIndex {
    idx: u32,
}

impl Deref for FaceIndex {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.idx
    }
}

impl HEIndex for FaceIndex {
    fn new(idx: u32) -> Self {
        FaceIndex {
            idx
        }
    }
}
