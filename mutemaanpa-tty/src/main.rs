use mutemaanpa_lib::game_state::{GameState, run};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("tty client started");
    let game_state = GameState::default();
    run(game_state);
}
