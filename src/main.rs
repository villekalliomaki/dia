#![forbid(unsafe_code)]
// Allow dead code in non-release builds.
#![cfg_attr(debug_assertions, allow(dead_code))]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate enum_display_derive;

mod access;
mod config;
mod db;
mod gql;
mod logging;
#[macro_use]
mod macros;
mod models;
mod res;
mod routes;

use crate::{
    access::{create_cors, RateLimiter, JWT},
    db::{RedisConn, SqlxConn},
    gql::build_schema,
};
use actix_web::{App, HttpServer};
pub use config::Config;
pub use res::Res;
use std::net::SocketAddr;

/// Static config file location. Replace with a CLI flag?
pub const CONF_FILE: &str = "./config.toml";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    logging::setup();

    // Application data, database clients
    let conf = Config::from_file(CONF_FILE);
    let pg = SqlxConn::new(&conf).await;
    let rd = RedisConn::new(&conf);
    let rl = RateLimiter::new(rd.clone());
    let schema = build_schema();
    let jwt = JWT::generate().unwrap();

    // Run Sqlx migrations
    pg.migrate().await;

    // Parse address and port to bind to
    let addr: SocketAddr = conf.bind_to.parse().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(create_cors())
            .data(schema.clone())
            .app_data(conf.clone())
            .app_data(pg.clone())
            .app_data(rd.clone())
            .app_data(rl.clone())
            .app_data(jwt.clone())
            .service(routes::build())
    })
    .bind(addr)?
    .run()
    .await
}
