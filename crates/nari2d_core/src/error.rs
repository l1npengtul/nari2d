use crate::asset::{AssetID, AssetType};
use crate::geometry::Point2d;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Nari2DError {
    #[error("Invalid Resource Operation {asset_id} of type {asset_type}: {error}")]
    InvalidAssetOperation {
        asset_id: AssetID,
        asset_type: AssetType,
        error: String,
    },
    #[error("No Resource {id}")]
    AssetNotFound { id: AssetID },
    #[error("Could not calculate concave for points {points}: {error}")]
    MeshConcaveCalculation { points: Vec<Point2d>, error: String },
    #[error("Points {points:?}, Errors during pre-processing: {error}")]
    MeshGenerationCleanup { points: Vec<Point2d>, error: String },
    #[error("Could not triangulate: {error}")]
    MeshTriangulation { error: String },
}

pub enum Nari2DResultCode {}

pub type NResult<T> = Result<T, Nari2DError>;
