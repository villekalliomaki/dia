use crate::models::{JwtMutation, RefreshTokenMutation, UserMutation};
use async_graphql::*;

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, RefreshTokenMutation, JwtMutation);
