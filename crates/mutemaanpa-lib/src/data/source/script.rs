//! Script data sources
//!

use assets_manager::Handle;

use super::ASSETS;

#[derive(Debug, Default)]
pub struct ScriptFileDataSource;

impl ScriptFileDataSource {
    pub fn new() -> ScriptFileDataSource {
        ScriptFileDataSource
    }

    pub fn get_script(&self, episode: &str) -> &'static Handle<String> {
        ASSETS.load(episode).unwrap()
    }
}
