use crate::resources::ResourceType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Nari2DError {
    InvalidIDError(u64),
    InvalidTypeError(ResourceType),
    InvalidPath(String),
}

pub enum Nari2DResultCode {}
