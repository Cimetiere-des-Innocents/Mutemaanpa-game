/// Call once tracing_subscriber::fmt::init() to initialize tracing.
#[cfg(test)]
mod tests_utils {
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn logging_init() {
        INIT.call_once(|| {
            tracing_subscriber::fmt::init();
        });
    }
}

pub mod class;
pub mod game_state;
pub mod i18n;
pub mod loader;
mod setting;
