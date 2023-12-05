use serde::{Deserialize, Serialize};

mod language;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Setting {
    pub language: language::LanguageSetting,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            language: language::LanguageSetting::default(),
        }
    }
}

impl Setting {
    pub const DEFAULT_SETTINGS_PATH: &'static str = "settings";

    pub fn load(path: &str) -> Result<Self, anyhow::Error> {
        let file = std::fs::File::open(path)?;
        let setting: Self = serde_lexpr::from_reader(file)?;
        Ok(setting)
    }

    pub fn save(&self, path: &str) -> Result<(), anyhow::Error> {
        let file = std::fs::File::create(path)?;
        serde_lexpr::to_writer(file, self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_setting() {
        let setting = Setting::default();
        let path = "test_setting";
        setting.save(path).unwrap();
        let loaded_setting = Setting::load(path).unwrap();
        assert_eq!(setting, loaded_setting);
        fs::remove_file(path).unwrap();
    }
}
