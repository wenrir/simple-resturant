//! application/config.rs

/// Default path to the app configuration file.
#[allow(unused)]
const DEFAULT_CONFIG_PATH: &str = if cfg!(debug_assertions) {
    concat!(env!("CARGO_MANIFEST_DIR"), "/config/config.toml")
} else {
    "./config.toml"
};
