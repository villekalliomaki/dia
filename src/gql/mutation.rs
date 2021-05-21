use crate::models::UserMutation;
use async_graphql::*;

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);
