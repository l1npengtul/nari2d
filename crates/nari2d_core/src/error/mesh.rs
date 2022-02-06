use crate::{
    error::util::{IndexOrValue, IndexType},
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
    #[error("Point not found at {idx}")]
    PointNotFound {
        idx: IndexOrValue<IndexType, Point2d>,
    },
    #[error("Point Relation not found for point {idx}")]
    NoPointRelation {
        idx: IndexOrValue<IndexType, Point2d>,
    },
    #[error("Edge not found at {idx}")]
    EdgeNotFound { edge: Edge },
    #[error("Point not found at {idx}")]
    TriangleNotFound {
        idx: IndexOrValue<TriangleRef, Triangle>,
    },
    #[error("Non Edge Operation on Triangle {triangle} with improper edge {edge}")]
    NonEdgeImproperEdge { triangle: Triangle, edge: Edge },
    #[error("Could not calculate concave hull: {why}")]
    ConcaveError { why: String },
    #[error("Too many points in mesh")]
    TooManyPoints,
    #[error("Less than 3 points in mesh")]
    LessThanThreePoints,
}

pub type MResult<T> = Result<T, MeshError>;
