use crate::{error::Nari2DError, traits::asset_service::AssetProvider};
use ahash::{AHasher, RandomState};
use dashmap::DashMap;
use gltf::Document;
use image::{ImageBuffer, Rgba};
use parking_lot::RwLock;
use std::{
    cell::Cell,
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

#[derive(Clone, Debug)]
pub struct NariAssetStore {
    asset_index: Cell<u32>,
    data_store: DashMap<AssetID, AssetData, RandomState>,
}

impl NariAssetStore {
    #[must_use]
    pub fn new() -> Self {
        NariAssetStore::default()
    }

    pub fn current(&self) -> u32 {
        self.asset_index.get()
    }

    pub fn increment(&self) -> u32 {
        let new = self.asset_index.get() + 1;
        self.asset_index.set(new);
        new
    }
}

impl Default for NariAssetStore {
    fn default() -> Self {
        NariAssetStore {
            asset_index: Cell::new(0),
            data_store: DashMap::with_hasher(RandomState::new()),
        }
    }
}

impl AssetProvider for NariAssetStore {
    fn add_asset_by_data(
        &self,
        res_type: AssetType,
        data: AssetRawHold,
    ) -> Result<AssetID, Nari2DError> {
        if self.current() == u32::MAX {
            return Err(Nari2DError::InvalidAssetOperation {
                asset_id: AssetID::new(self.current()),
                asset_type: res_type,
                error: "Max ID".to_string(),
            });
        }

        let id = AssetID::new(self.increment());

        let asset = AssetData::new(res_type, id, data, None);

        self.data_store.insert(id, asset);
        Ok(id)
    }

    fn add_asset_by_data_with_path(
        &self,
        res_type: AssetType,
        data: AssetRawHold,
        path: String,
    ) -> Result<AssetID, Nari2DError> {
        if self.current() == u32::MAX {
            return Err(Nari2DError::InvalidAssetOperation {
                asset_id: AssetID::new(self.current()),
                asset_type: res_type,
                error: "Max ID".to_string(),
            });
        }

        let id = AssetID::new(self.increment());

        let asset = AssetData::new(res_type, id, data, Some(path));

        self.data_store.insert(id, asset);
        Ok(id)
    }

    fn replace_reload_asset(&self, id: AssetID, data: AssetRawHold) -> Result<(), Nari2DError> {
        let asset = self.data(id);

        match asset {
            Some(res) => {
                let mut guard = res.write();
                *guard = data;
                Ok(())
            }
            None => Err(Nari2DError::AssetNotFound { id }),
        }
    }

    fn remove_asset(&self, asset_id: AssetID) {
        if let Some(mut asset) = self.data_store.get_mut(&asset_id) {
            (*asset).asset_type = AssetType::None;
            let mut write_lock = asset.data.write();
            *write_lock = AssetRawHold::None;
        }
    }

    fn data(&self, id: AssetID) -> Option<Arc<RwLock<AssetRawHold>>> {
        match self.data_store.get(&id) {
            Some(ds) => Some(ds.data.clone()),
            None => None,
        }
    }

    fn reference_count(&self, id: AssetID) -> Option<usize> {
        if let Some(asset) = self.data_store.get(&id) {
            return Some(asset.reference_count());
        }
        None
    }

    fn asset_type(&self, id: AssetID) -> Option<AssetType> {
        if let Some(asset) = self.data_store.get(&id) {
            return Some(asset.asset_type());
        }
        None
    }
}
