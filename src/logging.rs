use env_logger::{init_from_env, Env};

#[cfg(debug_assertions)]
const ENV_LEVEL: &'static str = "info";

#[cfg(not(debug_assertions))]
const ENV_LEVEL: &'static str = "warn";

/// Initialize env_logger. More verbose output when not running a release build.
pub fn setup() {
    init_from_env(Env::default().default_filter_or(ENV_LEVEL));
}
