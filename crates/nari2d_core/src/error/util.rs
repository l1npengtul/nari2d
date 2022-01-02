use std::fmt::{Debug, Display, Formatter};

pub enum IndexOrValue<I, V> {
    Index(I),
    Value(V),
}

impl<I, V> Debug for IndexOrValue<I, V>
where
    I: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexOrValue::Index(i) => {
                write!(f, "i:{:?}", i)
            }
            IndexOrValue::Value(v) => {
                write!(f, "v:{:?}", v)
            }
        }
    }
}

impl<I, V> Display for IndexOrValue<I, V>
where
    I: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum IndexType {
    U32(u32),
    Usize(usize),
}

impl From<u32> for IndexType {
    fn from(i: u32) -> Self {
        IndexType::U32(i)
    }
}

impl From<usize> for IndexType {
    fn from(i: usize) -> Self {
        IndexType::Usize(i)
    }
}

impl Debug for IndexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexType::U32(i) => {
                write!(f, "{}", i)
            }
            IndexType::Usize(i) => {
                write!(f, "{}", i)
            }
        }
    }
}

impl Display for IndexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
