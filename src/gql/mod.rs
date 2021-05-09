pub mod handlers;
mod mutation;
mod query;
mod subscription;

use async_graphql::{extensions::*, *};

pub type DiaSchema = Schema<query::Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> DiaSchema {
    Schema::build(query::Query, EmptyMutation, EmptySubscription)
        .data(())
        .extension(ApolloTracing)
        .extension(Analyzer)
        .extension(Tracing)
        .finish()
}
