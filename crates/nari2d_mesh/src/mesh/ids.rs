//! Defines unique id's for a vertex, half-edge and face.

use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Deref {
    /// Returns the inner value
    fn deref(&self) -> u32;
}

pub(crate) trait ID: Clone + Eq + Copy + Ord + Hash + Debug + Deref {
    fn new(val: u32) -> Self;
}

///
/// An unique ID for a vertex
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VertexID {
    val: u32,
}

impl ID for VertexID {
    fn new(val: u32) -> VertexID {
        VertexID { val }
    }
}

impl Deref for VertexID {
    fn deref(&self) -> u32 {
        self.val
    }
}

impl fmt::Display for VertexID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

///
/// An unique ID for a halfedge
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HalfEdgeID {
    val: u32,
}

impl ID for HalfEdgeID {
    fn new(val: u32) -> HalfEdgeID {
        HalfEdgeID { val }
    }
}

impl Deref for HalfEdgeID {
    fn deref(&self) -> u32 {
        self.val
    }
}

impl fmt::Display for HalfEdgeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

///
/// An unique ID for a face
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FaceID {
    val: u32,
}

impl ID for FaceID {
    fn new(val: u32) -> FaceID {
        FaceID { val }
    }
}

impl Deref for FaceID {
    fn deref(&self) -> u32 {
        self.val
    }
}

impl fmt::Display for FaceID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
