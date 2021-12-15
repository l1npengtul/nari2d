use std::{
    collections::BTreeMap,
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
};

pub trait HEIndex: Copy + Clone + Debug + Default + Deref + Hash + Eq + Ord + Send + Sync {
    fn new(idx: u32) -> Self;
}

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
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

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
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

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
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
        FaceIndex { idx }
    }
}
