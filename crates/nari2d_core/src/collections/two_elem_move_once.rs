use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    slice::from_raw_parts_mut,
    thread::sleep
};

/// A Vector that allows for grouping of 2 elements that only moves 1 at a time, **and** loops back to zero.
///
/// Ex:
/// ```.ignore
/// [1, 2, 3, 4, 5, 6, 7, 8] -> (1, 2), (2, 3), ..., (7, 8), (8, 1)
/// ```
pub struct TwoElemMoveOnceVec<T> {
    internal: Vec<T>,
    idx: usize,
}

impl<T> TwoElemMoveOnceVec<T> {
    pub fn new() -> Self {
        TwoElemMoveOnceVec {
            internal: vec![],
            idx: 0,
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        TwoElemMoveOnceVec {
            internal: Vec::with_capacity(size),
            idx: 0,
        }
    }
}

impl<T> Deref for TwoElemMoveOnceVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T> DerefMut for TwoElemMoveOnceVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}

impl<T> From<Vec<T>> for TwoElemMoveOnceVec<T> {
    fn from(src: Vec<T>) -> Self {
        TwoElemMoveOnceVec {
            internal: src,
            idx: 0,
        }
    }
}

impl<T> Into<Vec<T>> for TwoElemMoveOnceVec<T> {
    fn into(self) -> Vec<T> {
        self.internal
    }
}

impl<'a, T> Iterator for TwoElemMoveOnceVec<T> {
    type Item = (&'a T, &'a T);

    fn next(&'a mut self) -> Option<Self::Item> {
        let first_elem = match self.internal.get(self.idx) {
            Some(f) => f,
            None => return None,
        };

        let second_elem = match self.internal.get(self.idx) {
            Some(s) => s,
            None => match self.internal.get(self.idx) {
                Some(loop_value) => {
                    loop_value
                }
                None => return None,
            }
        };

        self.idx += 1;
        return Some((first_elem, second_elem))
    }
}
