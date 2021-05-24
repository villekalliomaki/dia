use crate::models::user::User;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
struct Claims {
    user: User,
    parent_token: String,
    iat: DateTime<Utc>,
    exp: DateTime<Utc>,
}
