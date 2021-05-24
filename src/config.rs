use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

/// App configuration. All fields must exists in the file.
#[derive(Deserialize, Clone)]
pub struct Config {
    pub bind_to: String,
    pub allow_registerations: bool,
    pub pg: PG,
    pub rd: RD,
}

/// PostgreSQL config options.
#[derive(Deserialize, Clone)]
pub struct PG {
    pub url: String,
    pub max_connections: u32,
}

/// Redis client configuration.
#[derive(Deserialize, Clone)]
pub struct RD {
    pub url: String,
}

impl Config {
    /// Creates a config from the specified file.
    /// Might panic with fs or parsing errors.
    pub fn from_file(path: &'static str) -> Config {
        from_str::<Config>(&read_to_string(path).unwrap()).unwrap()
    }
}

impl FromRequest for Config {
    type Error = ();
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.app_data::<Config>() {
            Some(conf) => ok(conf.clone()),
            _ => {
                log::error!("Config does not exists in app's data!");

                err(())
            }
        }
    }
}
