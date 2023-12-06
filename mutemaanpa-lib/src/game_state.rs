use crate::{class::ClassTree, i18n::I18nProvider, setting::Setting};
use tracing::info;

/// [`GameState`] stores states that shared by whole game.
pub struct GameState {
    setting: Setting,
    class_tree: ClassTree,
    pub i18n: I18nProvider,
}

impl Default for GameState {
    fn default() -> Self {
        let setting = match Setting::load(Setting::DEFAULT_SETTINGS_PATH) {
            Ok(setting) => setting,
            Err(err) => {
                info!("Did not find user setting, using default: {}", err);
                Setting::default()
            }
        };
        let i18n = I18nProvider::load(&setting.language.language).unwrap();
        Self {
            setting,
            class_tree: ClassTree::default(),
            i18n,
        }
    }
}

pub fn run(_: GameState) {
    info!("game started")
}

impl GameState {
    pub fn get_skill_tree(&self) -> &ClassTree {
        &self.class_tree
    }
}

#[test]
fn test_run() {
    let game_state = GameState::default();
    crate::tests_utils::logging_init();
    run(game_state);
}
