use crate::models::{Ping, Add};
use async_graphql::*;

#[derive(MergedObject, Default)]
pub struct Query(Ping, Add);
