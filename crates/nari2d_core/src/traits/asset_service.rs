use crate::{
    asset::{AssetID, AssetRawHold, AssetType},
    error::Nari2DError,
};
use parking_lot::RwLock;
use std::sync::Arc;

pub trait AssetProvider {
    /// # Errors
    fn add_asset_by_data(
        &self,
        res_type: AssetType,
        data: AssetRawHold,
    ) -> Result<AssetID, Nari2DError>;

    /// # Errors
    fn add_asset_by_data_with_path(
        &self,
        res_type: AssetType,
        data: AssetRawHold,
        path: String,
    ) -> Result<AssetID, Nari2DError>;

    /// # Errors
    fn replace_reload_asset(&self, id: AssetID, data: AssetRawHold) -> Result<(), Nari2DError>;

    fn remove_asset(&self, asset_id: AssetID);

    fn data(&self, id: AssetID) -> Option<Arc<RwLock<AssetRawHold>>>;

    fn reference_count(&self, id: AssetID) -> Option<usize>;

    fn asset_type(&self, id: AssetID) -> Option<AssetType>;
}
