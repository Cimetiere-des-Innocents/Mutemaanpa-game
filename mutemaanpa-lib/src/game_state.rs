use crate::setting::Setting;
use tracing::info;

/// [`GameState`] stores states that shared by whole game.
pub struct GameState {
    setting: Setting,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            setting: Setting::default(),
        }
    }
}

pub fn run(game_state: GameState) {
    info!("game started")
}

#[test]
fn test_run() {
    let game_state = GameState {
        setting: Setting::default(),
    };
    crate::tests_utils::logging_init();
    run(game_state);
}
