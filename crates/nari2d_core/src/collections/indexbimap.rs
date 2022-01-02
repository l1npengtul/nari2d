use ahash::RandomState;
use bimap::{
    hash::{IntoIter, Iter, LeftValues, RightValues},
    BiHashMap, Overwritten,
};
use nari2d_traits::index::NumIndex;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Formatter},
    hash::Hash,
};

pub type Indices<'a, I, V> = LeftValues<'a, I, V>;
pub type Values<'a, I, V> = RightValues<'a, I, V>;

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct IndexBiMap<I: NumIndex, V: Eq + Hash> {
    internal: BiHashMap<I, V, RandomState, RandomState>,
    free_indices: BTreeSet<I>,
    index: I,
}

impl<I, V> IndexBiMap<I, V>
where
    I: NumIndex,
    V: Eq + Hash,
{
    pub fn new() -> Self {
        IndexBiMap {
            internal: BiHashMap::default(),
            free_indices: BTreeSet::default(),
            index: I::zero(),
        }
    }

    pub fn with_capacity(capacity: I) -> Self {
        IndexBiMap {
            internal: BiHashMap::with_capacity_and_hashers(
                capacity.as_usize(),
                RandomState::new(),
                RandomState::new(),
            ),
            free_indices: BTreeSet::default(),
            index: I::zero(),
        }
    }

    pub fn with_data(data: impl IntoIterator<Item = V>) -> Self {
        let into_iter = data.into_iter();
        let amt = into_iter.count();
        let mut map = Self::with_capacity(amt as u32);
        into_iter.enumerate().for_each(|(idx, item)| {
            map.raw_insert(I::from(idx), item);
        });
        map
    }

    fn raw_insert(&mut self, index: I, value: V) {
        self.internal.insert(index, value);
        self.index = I::from(self.internal.len());
    }

    pub fn get_by_index(&self, index: &I) -> Option<&V> {
        self.internal.get_by_left(index)
    }

    pub fn get_by_value(&self, value: &V) -> Option<&I> {
        self.internal.get_by_right(value)
    }

    pub fn insert(&mut self, value: V) -> (I, Option<V>) {
        let index = match self.free_indices.first() {
            Some(idx) => *idx,
            None => {
                self.index += 1;
                (self.index - 1) // we get this value, NOT modify the struct!
            }
        };

        match self.internal.insert(index, value) {
            Overwritten::Neither | Overwritten::Left(_, _) | Overwritten::Pair(_, _) => {
                (index, None)
            }
            Overwritten::Both(_, (_, r)) | Overwritten::Right(_, r) => (index, Some(r)),
        }
    }

    pub fn remove_by_index(&mut self, index: I) -> Option<(I, V)> {
        match self.internal.remove_by_left(&index) {
            Some(p) => {
                self.free_indices.insert(p.0);
                Some(p)
            }
            None => None,
        }
    }

    pub fn remove_by_value(&mut self, value: V) -> Option<(I, V)> {
        match self.internal.remove_by_right(&value) {
            Some(p) => {
                self.free_indices.insert(p.0);
                Some(p)
            }
            None => None,
        }
    }

    pub fn iter<'a>(&self) -> Iter<'a, I, V> {
        self.internal.iter()
    }

    pub fn into_iter(self) -> IntoIter<I, V> {
        self.internal.into_iter()
    }

    pub fn indices<'a>(&self) -> Indices<'a, I, V> {
        self.internal.left_values()
    }

    pub fn values<'a>(&self) -> Values<'a, I, V> {
        self.internal.right_values()
    }

    pub fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<I, V> Clone for IndexBiMap<I, V>
where
    I: NumIndex,
    V: Clone + Eq + Hash,
{
    fn clone(&self) -> Self {
        IndexBiMap {
            internal: self.internal.clone(),
            free_indices: self.free_indices.clone(),
            index: self.index,
        }
    }
}

impl<I, V> Debug for IndexBiMap<I, V>
where
    I: NumIndex,
    V: Debug + Eq + Hash,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.internal.fmt(f)
    }
}

impl<I, V> PartialEq<Self> for IndexBiMap<I, V>
where
    I: NumIndex,
    V: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.internal.eq(&other.internal)
            && self.index.eq(&other.index)
            && self.free_indices.eq(&other.free_indices)
    }
}

impl<I, V> Eq for IndexBiMap<I, V>
where
    I: NumIndex,
    V: Eq + Hash,
{
}

// The underlying store BiHashMap is Send + Sync if L and R are both Send + Sync. Since L is Send + Sync by default (usize), we only need to check if V(R) fits the conditions.
unsafe impl<I, V> Send for IndexBiMap<I, V>
where
    I: Send + NumIndex,
    V: Send,
{
}
unsafe impl<I, V> Sync for IndexBiMap<I, V>
where
    I: Sync + NumIndex,
    V: Sync,
{
}
