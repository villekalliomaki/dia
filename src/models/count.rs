use async_graphql::*;
use futures_core::stream::Stream;

#[derive(Default)]
pub struct CountSubscription;

#[Subscription]
impl CountSubscription {
    /// Test subscriptions.
    async fn count_to_10(&self) -> impl Stream<Item = i32> {
        futures::stream::iter(0..10)
    }
}
