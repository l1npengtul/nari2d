use crate::error::Nari2DError;
use dashmap::DashMap;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

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

impl Deref for ResourceID {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ResourceID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ResourceID> for u32 {
    fn from(id: ResourceID) -> Self {
        id.0
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ResourcePath {
    Local { data: SmallVec<[String; 8]> },
    External { path: PathBuf },
}

impl ResourcePath {
    pub fn new(path: String) -> Self {
        let mut path = path;
        if path.starts_with("n2dfile://") {
            path.replace("n2dfile://", "");
            let mut directories = vec![];
            for dir in path.split("/") {
                directories.push(dir.to_string());
            }
            ResourcePath::Local {
                data: SmallVec::from_vec(directories),
            }
        } else {
            ResourcePath::External {
                path: PathBuf::from(path),
            }
        }
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResourceData<'a> {
    reference_count: AtomicUsize,
    resource_type: ResourceType,
    resource_id: u32,
    data: Arc<RwLock<Cow<'a, [u8]>>>,
    source: ResourcePath,
}

impl<'a> ResourceData<'a> {}

#[derive(Clone, Debug)]
pub struct ResourceStore {
    store_path: &'static str,
    data_store: DashMap<ResourceID, ResourceData<'static>>,
}

impl ResourceStore {}
