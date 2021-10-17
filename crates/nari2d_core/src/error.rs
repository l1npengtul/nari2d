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
    #[error("Points {points:?} invalid: {error}")]
    InvalidMesh { points: Vec<Point2d>, error: String },
}

pub enum Nari2DResultCode {}

pub type NResult<T> = Result<T, Nari2DError>;
