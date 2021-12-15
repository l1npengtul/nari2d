use crate::geometry::{
    half_edge::index::{FaceIndex, HalfEdgeIndex, VertexIndex},
    Point2d,
};

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HPoint {
    pub halfedge: Option<HalfEdgeIndex>,
    pub position: Point2d,
}

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct HalfEdge {
    pub vertex: Option<VertexIndex>,
    pub twin: Option<HalfEdgeIndex>,
    pub next: Option<HalfEdgeIndex>,
    pub face: Option<FaceIndex>,
}

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Face {
    pub halfedge: Option<VertexIndex>,
}
