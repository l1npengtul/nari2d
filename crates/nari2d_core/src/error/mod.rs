pub mod asset;
pub mod mesh;
pub mod util;

use crate::{
    asset::{AssetID, AssetType},
    error::{asset::AssetError, mesh::MeshError},
};
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Nari2DError {
    #[error("Asset Error: ID {asset_id} of type {asset_type}: {error}")]
    Asset {
        asset_id: AssetID,
        asset_type: AssetType,
        error: AssetError,
    },
    #[error("Mesh Error: {error}")]
    Mesh { error: MeshError },
}

impl From<MeshError> for Nari2DError {
    fn from(m: MeshError) -> Self {
        Nari2DError::Mesh { error: m }
    }
}

pub type NResult<T> = Result<T, Nari2DError>;
