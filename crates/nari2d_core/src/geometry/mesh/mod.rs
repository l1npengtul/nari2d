// use crate::{
//     collections::point_store::PointStore,
//     error::mesh::{MResult, MeshError},
//     geometry::{Angle, IndexedPoint2d, Orientation, Point2d},
// };
// use std::{cmp::Ordering, ops::Add};

use smallvec::SmallVec;
use std::fmt::{Display, Formatter};

#[cfg(feature = "edit")]
pub mod edit_mesh;
pub mod simple_mesh;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Triangle {
    pub point0: PointId,
    pub point1: PointId,
    pub point2: PointId,
}

impl Triangle {
    pub fn contains(&self, id: PointId) -> bool {
        self.point0 == id || self.point1 == id || self.point2 == id
    }
}

impl From<[PointId; 3]> for Triangle {
    fn from(pts: [PointId; 3]) -> Self {
        Self {
            point0: pts[0],
            point1: pts[1],
            point2: pts[2],
        }
    }
}

impl From<(PointId, PointId, PointId)> for Triangle {
    fn from(pts: (PointId, PointId, PointId)) -> Self {
        Self {
            point0: pts.0,
            point1: pts.1,
            point2: pts.2,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Edge {
    pub point0: PointId,
    pub point1: PointId,
    pub triangle0: Option<TriangleId>,
    pub triangle1: Option<TriangleId>,
}

impl Edge {
    pub fn contains_point(&self, point: PointId) -> bool {
        self.point0 == point || self.point1 == point
    }

    pub fn add_triangle_unoccupied(&mut self, new: TriangleId) -> Option<()> {
        match self.triangle0 {
            None => {
                self.triangle0 = Some(new);
            }
            Some(_) => match self.triangle1 {
                None => {
                    self.triangle1 = Some(new);
                }
                Some(_) => return None,
            },
        };
        Some(())
    }
}

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct PointEdge {
    pub edges: SmallVec<[EdgeId; 6]>,
}

impl PointEdge {
    pub fn edges(&self) -> &[EdgeId] {
        &self.edges
    }
}

impl From<SmallVec<[EdgeId; 6]>> for PointEdge {
    fn from(sv: SmallVec<[EdgeId; 6]>) -> Self {
        Self { edges: sv }
    }
}

impl From<[EdgeId; 6]> for PointEdge {
    fn from(array: [EdgeId; 6]) -> Self {
        Self {
            edges: SmallVec::from(array),
        }
    }
}

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct TriangleEdge {
    pub edges: [EdgeId; 3],
}

impl From<[EdgeId; 3]> for TriangleEdge {
    fn from(sv: [EdgeId; 3]) -> Self {
        Self { edges: sv }
    }
}

slotmap::new_key_type! { pub struct PointId; }
slotmap::new_key_type! { pub struct TriangleId; }
slotmap::new_key_type! { pub struct EdgeId; }

impl Display for PointId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for TriangleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for EdgeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl triangle mesh
pub trait MeshImplementation {}
