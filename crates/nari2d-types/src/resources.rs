use crate::error::Nari2DError;
use std::borrow::Cow;
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

impl ResourceID {}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResourceData<'a> {
    reference_count: AtomicUsize,
    resource_type: ResourceType,
    resource_id: u32,
    data: Cow<'a, [u8]>,
}

impl<'a> ResourceData<'a> {}
