use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameplaySetting {
    pub entry_point: String,
}

impl Default for GameplaySetting {
    fn default() -> Self {
        Self {
            entry_point: "prologue".to_string(),
        }
    }
}
