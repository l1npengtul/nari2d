use std::{
    collections::BTreeMap,
    fmt::Debug,
    hash::Hash,
    ops::Deref
};
use crate::geometry::half_edge::index::HEIndex;

pub trait HEValue: Copy + Clone + Debug + Default + Deref + Hash + Eq + Ord + Send + Sync;

// TODO: just use tri-mesh but make it 2D

pub struct IndexMap<I, V> where
I: HEIndex,
V: HEValue,
{
    backing: BTreeMap<I, V>
}

impl<I: HEIndex + 'static, V> IndexMap<I, V> where
    I: HEIndex,
    V: HEValue,
{
    pub fn new() -> Self {
        IndexMap {
            backing: Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn insert(&mut self, value: V) -> Option<I> {

    }
}
