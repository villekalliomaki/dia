#![forbid(unsafe_code)]

mod config;
mod db;
mod gql;
mod logging;
mod models;
mod res;
mod routes;

use crate::{
    config::Config,
    db::{redis::create_redis_client, sqlx::create_sqlx_pool},
    gql::build_schema,
};
use actix_web::{App, HttpServer};
use std::net::SocketAddr;

const CONF_FILE: &str = "./config.toml";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    logging::setup();

    // Application data, database clients
    let conf = Config::from_file(CONF_FILE);
    let pg = create_sqlx_pool(&conf).await;
    let rd = create_redis_client(&conf);
    let schema = build_schema();

    // Parse address and port to bind to
    let addr: SocketAddr = conf.bind_to.parse().unwrap();

    HttpServer::new(move || {
        App::new()
            .data(conf.clone())
            .data(schema.clone())
            .data(pg.clone())
            .data(rd.clone())
            .service(routes::build())
    })
    .bind(addr)?
    .run()
    .await
}
