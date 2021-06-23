use crate::{
    access::jwt::{JwtClaims, JWT},
    models::{refresh_token::RefreshToken, user::User},
};

use async_graphql::*;
use chrono::{Duration, Utc};

#[derive(Default)]
pub struct JwtMutation;

#[Object]
impl JwtMutation {
    /// Generate a new JWT from a token string.
    /// Lifetime must be equal or lower as the tokens maximum JWT lifetime.
    async fn sign_jwt(
        &self,
        ctx: &Context<'_>,
        refresh_token_string: String,
        #[graphql(default = 300)] lifetime: i64,
    ) -> Result<String> {
        let pool = ctx.data::<sqlx::PgPool>()?;

        // Get the token the string refers to
        let refresh_token = sqlx::query_as!(
            RefreshToken,
            "SELECT * FROM refresh_tokens WHERE expires > NOW() AND token_string = $1;",
            refresh_token_string
        )
        .fetch_one(pool)
        .await?;

        // Check that the lifetime of an allowed length
        if lifetime > refresh_token.max_jwt_lifetime as i64 {
            return Err(Error::new(format!(
                "Requested JWT lifetime exceeds the limit of {}.",
                refresh_token.max_jwt_lifetime,
            )));
        }

        // Get the user correspoding to the refresh token
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            refresh_token.user_id
        )
        .fetch_one(pool)
        .await?;

        let claims = JwtClaims {
            user,
            parent_token: refresh_token.id,
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::seconds(lifetime)).timestamp(),
        };

        Ok(ctx.data::<JWT>()?.encode(&claims)?)
    }
}
