use super::User;

use crate::Config;
use async_graphql::*;
use tokio::task::spawn_blocking;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get the user with the correct credentials.
    async fn user(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", &username)
            .fetch_one(ctx.data::<sqlx::PgPool>()?)
            .await?;

        let c = user.clone();

        spawn_blocking(move || return c.validate_password(password)).await??;

        Ok(user)
    }

    /// `true` if the creation of new users is enabled.
    async fn registerations_allowed(&self, ctx: &Context<'_>) -> Result<bool> {
        Ok(ctx.data::<Config>()?.allow_registerations)
    }
}
