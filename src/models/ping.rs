use async_graphql::*;

#[derive(Default)]
pub struct Ping;

#[Object]
impl Ping {
    /// Ping the GQL server.
    async fn ping(&self, ctx: &Context<'_>) -> String {
        let _ = ctx.data::<String>();

        "pong".to_string()
    }
}
