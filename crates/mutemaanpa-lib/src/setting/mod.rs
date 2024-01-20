use serde::{Deserialize, Serialize};

use self::{language::LanguageSetting, gameplay::GameplaySetting};

mod language;
mod gameplay;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Setting {
    pub language: language::LanguageSetting,
    pub gameplay: gameplay::GameplaySetting,
    path: String,
}

impl Default for Setting {
    fn default() -> Setting {
        Self::new(Self::DEFAULT_SETTINGS_PATH)
    }
}

impl Setting {
    pub const DEFAULT_SETTINGS_PATH: &'static str = "settings.cfg";

    pub fn new(path: &str) -> Setting {
        Self {
            language: LanguageSetting::default(),
            gameplay: GameplaySetting::default(),
            path: path.to_string(),
        }
    }

    pub fn load(path: &str) -> Result<Self, anyhow::Error> {
        let file = std::fs::File::open(path)?;
        let setting: Self = serde_lexpr::from_reader(file)?;
        Ok(setting)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let file = std::fs::File::create(self.path.as_str())?;
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
        let path = "test_setting";
        let setting = Setting::new(path);
        setting.save().unwrap();
        let loaded_setting = Setting::load(path).unwrap();
        assert_eq!(setting, loaded_setting);
        fs::remove_file(path).unwrap();
    }
}
