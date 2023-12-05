use mutemaanpa_lib::game_state::{run, GameState};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("tty client started");
    let game_state = GameState::default();
    run(game_state);
}
