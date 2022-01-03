use crate::collections::indexbimap::IndexBiMap;
use nari2d_traits::index::NumIndex;
use rstar::{RTree, RTreeObject};
use std::hash::Hash;
use std::rc::Rc;

impl {

}

pub struct PointStore<I: NumIndex, V: RTreeObject + Eq + Hash> {
    index_store: IndexBiMap<I, Rc<V>>, // This field will "own" the Rc
    rtree_store: RTree<Rc<V>>,         // This field will "borrow" the Rc
}

impl<I, V> PointStore<I, V> where
    I: NumIndex,
    V: RTreeObject + Eq + Hash,
{
    pub fn new() -> Self {
        PointStore {
            index_store: IndexBiMap::default(),
            rtree_store: RTree::default(),
        }
    }

    pub fn insert(&mut self, point: V) -> I {
        let rc_owned = Rc::new(point);
        let index = self.index_store.insert(rc_owned);
        self.rtree_store.
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
            let rc_owned: Rc<V> = Rc::new(kv.1);
            let rc_borrow = rc_owned.clone();
            indices.insert(rc_owned);
            values.push(rc_borrow);
        });
        let rtree = RTree::bulk_load(values);
        PointStore {
            index_store: indices,
            rtree_store: rtree,
        }
    }
}
