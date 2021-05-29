use crate::models::{Add, JwtQuery, Ping, UserQuery};
use async_graphql::*;

#[derive(MergedObject, Default)]
pub struct Query(Ping, Add, UserQuery, JwtQuery);
