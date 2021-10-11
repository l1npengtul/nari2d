use crate::asset::{AssetID, AssetType};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Nari2DError {
    #[error("Invalid Resource Operation {resource_id} of type {resource_type}: {error}")]
    InvalidAssetOperation {
        resource_id: AssetID,
        resource_type: AssetType,
        error: String,
    },
    #[error("No Resource {id}")]
    AssetNotFound { id: AssetID },
}

pub enum Nari2DResultCode {}
