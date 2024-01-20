//! Script data sources
//!

use assets_manager::{
    loader::{self, StringLoader},
    Asset, Handle,
};

use super::ASSETS;

#[derive(Clone, Debug)]
pub struct Script(pub String);

impl From<String> for Script {
    fn from(value: String) -> Self {
        Script(value)
    }
}

impl Asset for Script {
    const EXTENSION: &'static str = "mdl";

    type Loader = loader::LoadFrom<String, StringLoader>;
}

#[derive(Debug, Default)]
pub struct ScriptFileDataSource;

impl ScriptFileDataSource {
    pub fn new() -> ScriptFileDataSource {
        ScriptFileDataSource
    }

    pub fn get_script(&self, episode: &str) -> &'static Handle<Script> {
        let script_addr = ["script.", episode].concat();
        ASSETS.load(&script_addr).unwrap()
    }
}
