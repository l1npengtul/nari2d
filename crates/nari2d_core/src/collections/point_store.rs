use crate::collections::indexbimap::{IndexBiMap, Indices, Values};
use nari2d_traits::index::NumIndex;
use rstar::{Point, RTree, RTreeObject};
use std::{fmt::Debug, hash::Hash, ops::Deref};

// just store it twice. What's the user going to do, store 100K+ points?
// The overhead from storing the point list twice is probably lower anyway, since Rc contains 2 `usize`.
// TODO: Implement an r*-tree with index built in order to not have to store things twice.
pub struct PointStore<I: NumIndex, V: RTreeObject + Point + Eq + Hash> {
    index_store: IndexBiMap<I, V>,
    rtree_store: RTree<V>,
}

impl<I, V> PointStore<I, V>
where
    I: NumIndex,
    V: RTreeObject + Point + Eq + Hash,
{
    pub fn new() -> Self {
        PointStore {
            index_store: IndexBiMap::default(),
            rtree_store: RTree::default(),
        }
    }

    pub fn insert(&mut self, point: V) -> (I, Option<V>) {
        let index = self.index_store.insert(point);
        self.rtree_store.insert(point);
        index
    }

    pub fn get_by_index(&self, index: &I) -> Option<&V> {
        self.index_store.get_by_index(index).map(|x| &*x.0)
    }

    pub fn get_by_value(&self, value: &V) -> Option<&I> {
        self.index_store.get_by_value(value)
    }

    pub fn remove_by_index(&mut self, index: &I) -> Option<(I, V)> {
        match self.index_store.remove_by_index(index) {
            Some((index, pointref)) => {
                self.rtree_store.remove(&pointref);
                Some((index, pointref))
            }
            None => None,
        }
    }

    pub fn remove_by_value(&mut self, value: V) -> Option<(I, V)> {
        match self.index_store.remove_by_value(value) {
            Some((index, point)) => {
                self.rtree_store.remove(&pointref);
                Some((index, point))
            }
            None => None,
        }
    }

    pub fn indices<'a>(&self) -> Indices<'a, I, V> {
        self.index_store.left_values()
    }

    pub fn values<'a>(&self) -> Values<'a, I, V> {
        self.index_store.right_values()
    }
}

impl<I, V> Deref for PointStore<I, V> {
    type Target = RTree<V>;

    fn deref(&self) -> &Self::Target {
        &self.rtree_store
    }
}

impl<I, V> From<Vec<V>> for PointStore<I, V>
where
    I: NumIndex,
    V: RTreeObject + Eq + Hash + Clone,
{
    fn from(src: Vec<V>) -> Self {
        let index_store = IndexBiMap::with_data(src.clone());
        let rtree_store = RTree::bulk_load(src);

        PointStore {
            index_store,
            rtree_store,
        }
    }
}

impl<I, V> FromIterator<(I, V)> for PointStore<I, V>
where
    I: NumIndex,
    V: RTreeObject + Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (I, V)>>(iter: T) -> Self {
        let into_iter = iter.into_iter();
        let mut indices: IndexBiMap<I, V> = IndexBiMap::with_capacity(into_iter.count());
        let mut values = Vec::with_capacity(into_iter.count());
        into_iter.for_each(|kv| {
            indices.insert(kv.1);
            values.push(rc_borrow);
        });
        let rtree = RTree::bulk_load(values);
        PointStore {
            index_store: indices,
            rtree_store: rtree,
        }
    }
}
