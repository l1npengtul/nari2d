use crate::error::Nari2DError;
use dashmap::DashMap;
use gltf::Scene;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::{
    borrow::Cow,
    convert::TryFrom,
    ffi::OsString,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::{atomic::AtomicUsize, Arc},
};

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ResourceFileType {
    ImagePNG,
    ImageTIFF,
    ImageJPEG,
    ImageBMP,
    MeshGLTF,
}

pub enum ResourceRawHold<'a> {
    ImageRGBA {
        width: u32,
        height: u32,
        data: Cow<'a, [u8]>,
    },
    MeshGLTF {
        scene: Scene<'a>,
    },
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResourceID(u32);

impl ResourceID {}

impl From<ResourceID> for u32 {
    fn from(id: ResourceID) -> Self {
        id.0
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResourceData<'a> {
    reference_count: AtomicUsize,
    resource_type: ResourceFileType,
    resource_id: u32,
    data: Arc<RwLock<Cow<'a, [u8]>>>,
    source: SmallVec<[String; 8]>,
}

impl<'a> ResourceData<'a> {}

#[derive(Clone, Debug)]
pub struct ResourceStore {
    store_path: &'static str,
    data_store: DashMap<ResourceID, ResourceData<'static>>,
}

impl ResourceStore {}
