use image::{ImageBuffer, Rgba};
use parking_lot::RwLock;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum AssetType {
    None,
    Image,
}

impl Display for AssetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub enum AssetRawHold {
    ImageRGBA {
        width: u32,
        height: u32,
        data: ImageBuffer<Rgba<u8>, Vec<u8>>,
    },
    None,
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct AssetID(u32);

impl AssetID {
    #[must_use]
    pub fn new(id: u32) -> Self {
        AssetID(id)
    }
}

impl Display for AssetID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AssetID> for u32 {
    fn from(id: AssetID) -> Self {
        id.0
    }
}

#[derive(Clone, Debug)]
pub struct AssetData {
    asset_type: AssetType,
    asset_id: AssetID,
    data: Arc<RwLock<AssetRawHold>>,
    path: Option<String>,
}

impl AssetData {
    #[must_use]
    pub fn new(
        asset_type: AssetType,
        asset_id: AssetID,
        data: AssetRawHold,
        path: Option<String>,
    ) -> Self {
        AssetData {
            asset_type,
            asset_id,
            data: Arc::new(RwLock::new(data)),
            path,
        }
    }

    #[must_use]
    pub fn reference_count(&self) -> usize {
        Arc::weak_count(&self.data)
    }

    #[must_use]
    pub fn asset_type(&self) -> AssetType {
        self.asset_type
    }

    #[must_use]
    pub fn data(&self) -> Arc<RwLock<AssetRawHold>> {
        self.data.clone()
    }

    #[must_use]
    pub fn id(&self) -> AssetID {
        self.asset_id
    }

    #[must_use]
    pub fn path(&self) -> Option<&str> {
        match &self.path {
            Some(path) => Some(path),
            None => None,
        }
    }
}
