use crate::resources::Nari2DType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Nari2DError {
    InvalidIDError(u64),
    InvalidTypeError(Nari2DType),
}

pub enum Nari2DResultCode {}
