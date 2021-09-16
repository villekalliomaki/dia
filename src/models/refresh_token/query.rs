use super::RefreshToken;

use crate::{gql::E, models::user::User};

use async_graphql::*;

#[derive(Default)]
pub struct RefreshTokenQuery;

#[Object]
impl RefreshTokenQuery {
    /// Get a single non expired refresh token from a string.
    async fn refresh_token_from_token_string(
        &self,
        ctx: &Context<'_>,
        token_string: String,
    ) -> std::result::Result<RefreshToken, E> {
        Ok(sqlx::query_as!(
            RefreshToken,
            "SELECT * FROM refresh_tokens WHERE expires > NOW() AND token_string = $1;",
            token_string
        )
        .fetch_one(ctx.data::<sqlx::PgPool>()?)
        .await?)
    }

    /// Get all user's refresh tokens. If valid is `true`, returns only usable tokens.
    async fn refresh_tokens(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
        #[graphql(default = true)] valid: bool,
    ) -> std::result::Result<Vec<RefreshToken>, E> {
        let pool = ctx.data::<sqlx::PgPool>()?;

        let user = User::from_credentials(pool, username, password).await?;

        Ok(sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT * FROM refresh_tokens
            WHERE (CASE WHEN $1 THEN expires > NOW() END)
                AND user_id = $2 ORDER BY created;
            "#,
            valid,
            user.id
        )
        .fetch_all(pool)
        .await?)
    }
}
