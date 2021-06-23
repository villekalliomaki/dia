use crate::models::CountSubscription;
use async_graphql::*;

#[derive(MergedSubscription, Default)]
pub struct Subscription(CountSubscription);
