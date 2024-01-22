use assets_manager::AssetCache;
use once_cell::sync::Lazy;

/// We are in /crates/mutemaanpa-lib
/// Assets folder are in /assets
/// So the relative path is:
pub const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets");
pub mod language;
pub mod script;

/// Data Source from the Asset folder
pub static ASSETS: Lazy<AssetCache> = Lazy::new(|| AssetCache::new(ASSETS_DIR).unwrap());
