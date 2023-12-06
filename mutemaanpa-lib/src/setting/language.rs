use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LanguageSetting {
    pub language: String,
}

impl Default for LanguageSetting {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
        }
    }
}
