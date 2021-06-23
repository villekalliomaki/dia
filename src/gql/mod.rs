mod mutation;
mod query;
mod subscription;

use async_graphql::{extensions::*, *};

pub type DiaSchema = Schema<query::Query, mutation::Mutation, subscription::Subscription>;

pub fn build_schema() -> DiaSchema {
    Schema::build(
        query::Query::default(),
        mutation::Mutation::default(),
        subscription::Subscription::default(),
    )
    .data(())
    .extension(ApolloTracing)
    .extension(Analyzer)
    .finish()
}
