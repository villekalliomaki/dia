mod config;
mod gql;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    server::run().await;
}
