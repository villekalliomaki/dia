use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

/**
 * App configuration. All fields must exists in the file.
 */
#[derive(Deserialize, Clone)]
pub struct Config {
    pub bind_to: String,
    pub pg: PG,
    pub rd: RD,
}

/**
 * PostgreSQL config options.
 */
#[derive(Deserialize, Clone)]
pub struct PG {
    pub url: String,
    pub max_connections: u32,
}

/**
 * Redis client configuration.
 */
#[derive(Deserialize, Clone)]
pub struct RD {
    pub url: String,
}

impl Config {
    /**
     * Creates a config from the specified file.
     * Might panic with fs or parsing errors.
     */
    pub fn from_file(path: &'static str) -> Config {
        from_str::<Config>(&read_to_string(path).unwrap()).unwrap()
    }
}
