use anyhow::Result;
use assets_manager::{AssetCache, Compound};
use once_cell::sync::Lazy;
use tracing::info;

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets");

/// Global repository of assets.
static ASSETS: Lazy<AssetCache> = Lazy::new(|| {
    info!("Loading assets from {}", ASSETS_DIR);
    AssetCache::new(ASSETS_DIR).unwrap()
});

pub fn start_hot_reload() {
    info!("asset hot reload started");
    ASSETS.enhance_hot_reloading();
}

pub trait GameAsset: Sized + Sync + Send + 'static {
    fn load(uri: &str) -> Result<assets_manager::Handle<'static, Self>>;
}

impl<T> GameAsset for T
where
    T: Compound,
{
    fn load(uri: &str) -> Result<assets_manager::Handle<'static, Self>> {
        ASSETS.load(uri).map_err(Into::into)
    }
}
