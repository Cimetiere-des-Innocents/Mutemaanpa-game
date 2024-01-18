use assets_manager::AssetCache;
use once_cell::sync::Lazy;

pub const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets");
pub mod language;

/// Data Source from the Asset folder
pub static ASSETS: Lazy<AssetCache> = Lazy::new(|| AssetCache::new(ASSETS_DIR).unwrap());
