use crate::{
    error::{
        util::{IndexOrValue, IndexType},
        IndexOrValue,
    },
    geometry::{
        mesh::{Edge, Triangle, TriangleRef},
        Point2d,
    },
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Failed to triangulate mesh: {why}")]
    Triangulation { why: String },
    #[error("Point not found at ")]
    PointNotFound {
        idx: IndexOrValue<IndexType, Point2d>,
    },
    #[error("Edge not found at {idx}")]
    EdgeNotFound { idx: IndexOrValue<IndexType, Edge> },
    #[error("Point not found at {idx}")]
    TriangleNotFound {
        idx: IndexOrValue<TriangleRef, Triangle>,
    },
    #[error("Non Edge Operation on Triangle {triangle} with improper edge {edge}.")]
    NonEdgeImproperEdge { triangle: Triangle, edge: Edge },
}

pub type MResult<T> = Result<T, MeshError>;
