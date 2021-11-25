use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IndexMap<I, V>
where
    I: PartialOrd + PartialEq + Clone + Debug,
    V: PartialOrd + PartialEq + Clone + Debug, {
    indexes:
}
