use crate::{config::Config, db::sqlx::create_sqlx_pool, routes::root};
use std::net::SocketAddr;

const CONF_FILE: &'static str = "./config.toml";

/**
 * Run the server asyncronously.
 * Easier than returning the warp server with generics.
 */
pub async fn run() {
    let conf = Config::from_file(CONF_FILE);

    let _pg = create_sqlx_pool(&conf).await.unwrap();

    let socket_address: SocketAddr = conf.bind_to.parse().unwrap();

    warp::serve(root()).run(socket_address).await;
}
