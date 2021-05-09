use crate::{config::Config, routes::root};
use std::net::SocketAddr;

const CONF_FILE: &'static str = "./config.toml";

pub async fn run() {
    let conf = Config::from_file(CONF_FILE);

    let socket_address: SocketAddr = conf.bind_to.parse().unwrap();

    warp::serve(root()).run(socket_address).await;
}
