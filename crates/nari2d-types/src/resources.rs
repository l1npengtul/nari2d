use crate::error::Nari2DError;
use std::convert::TryFrom;
use std::sync::atomic::AtomicUsize;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ResourceType {
    ImagePNG,
    ImageTIFF,
    ImageJPEG,
    ImageBMP,
    MeshGLTF,
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResourceID(u32);

pub enum ResourceLocator {}

pub struct ResourceData {
    reference_count: AtomicUsize,
}
