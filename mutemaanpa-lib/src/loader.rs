use anyhow::Result;
use assets_manager::{AssetCache, Compound};
use once_cell::sync::Lazy;

const ASSETS_DIR: &str = "../assets";

/// Global repository of assets.
static ASSETS: Lazy<AssetCache> = Lazy::new(|| AssetCache::new(ASSETS_DIR).unwrap());

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
