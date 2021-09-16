use async_graphql::*;

#[derive(Default)]
pub struct Ping;

#[Object]
impl Ping {
    /// Ping the GQL server.
    async fn ping(&self) -> &'static str {
        "pong"
    }
}
