use crate::error::Nari2DError;
use std::convert::TryFrom;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Nari2DType {
    Node = 0,
    Scene = 1,
    ResourceJPG = 2,
    ResourcePNG = 3,
    ResourceTIFF = 4,
    ResourceORA = 5,
    ResourceBIN = 6,
    Resource3DMesh = 7,
    Resource3DShader = 8,
    Invalid = 255,
}
