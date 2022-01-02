use num_traits::Num;
use std::{fmt::Debug, hash::Hash};

pub trait NumIndex:
    Copy + Clone + Debug + Default + Hash + Eq + PartialEq + Ord + PartialOrd + Num
{
    fn as_usize(self) -> usize;
    fn from_usize(i: usize) -> Self;
}

impl NumIndex for u8 {
    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(i: usize) -> Self {
        i as u8
    }
}

impl NumIndex for u16 {
    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(i: usize) -> Self {
        i as u16
    }
}

impl NumIndex for u32 {
    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(i: usize) -> Self {
        i as u32
    }
}

impl NumIndex for usize {
    fn as_usize(self) -> usize {
        self
    }

    fn from_usize(i: usize) -> Self {
        i
    }
}
