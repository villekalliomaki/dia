mod config;
mod ctx;
mod db;
mod gql;
mod handlers;
mod res;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    log::info!("Starting DIA");

    server::run().await;
}
