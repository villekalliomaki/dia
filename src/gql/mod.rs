mod gql_result;
mod mutation;
mod query;
mod subscription;

pub use gql_result::{E, R};

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
