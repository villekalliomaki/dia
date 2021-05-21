use super::User;

use anyhow::{anyhow, bail};
use async_graphql::*;
use tokio::task::spawn_blocking;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> async_graphql::Result<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", &username)
            .fetch_one(ctx.data::<sqlx::PgPool>()?)
            .await?;

        let c = user.clone();

        spawn_blocking(move || return c.validate_password(password)).await??;

        Ok(user)
    }
}
