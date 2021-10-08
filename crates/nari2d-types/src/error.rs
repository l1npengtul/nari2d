use crate::resources::ResourceFileType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Nari2DError {
    InvalidIDError(u64),
    InvalidTypeError(ResourceFileType),
    InvalidPath(String),
}

pub enum Nari2DResultCode {}
