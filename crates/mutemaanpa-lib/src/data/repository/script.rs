//!
//! # Script repository
//!
//! Lookup script name in data sources
//!

use tracing::info;

use crate::data::source::script::{Script, ScriptFileDataSource};

pub struct ScriptRepository {
    source: ScriptFileDataSource,
}

impl ScriptRepository {
    pub fn new(source: ScriptFileDataSource) -> ScriptRepository {
        ScriptRepository { source }
    }

    pub fn get_script(&mut self, name: &str) -> Script {
        let s = self.source.get_script(name).read().to_owned();
        info!("script {} is loaded", name);
        s
    }
}
