use super::RefreshToken;

use crate::{gql::E, models::user::User};

use async_graphql::*;
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use std::net::IpAddr;
use validator::Validate;

/// User credentials and lifetime for a new refresh token.
#[derive(Validate, InputObject, Clone)]
struct NewRefreshToken {
    username: String,
    password: String,
    /// The timespan from now the token is valid for in seconds.
    /// Defaults to 1 week.
    /// Between a minute and a month.
    #[graphql(default = 604800)]
    #[validate(range(min = 60, max = 2629800))]
    expires_in_seconds: i32,
    /// The maximum allowed lifetime for JWTs generated with this token in seconds.
    /// Between 10 seconds and an hour (3600 seconds).
    #[graphql(default = 300)]
    #[validate(range(min = 10, max = 3600))]
    max_jwt_lifetime: i32,
}

#[derive(Default)]
pub struct RefreshTokenMutation;

#[Object]
impl RefreshTokenMutation {
    // Create a new refresh token with the user's credentials.
    async fn create_refresh_token(
        &self,
        ctx: &Context<'_>,
        new_token: NewRefreshToken,
    ) -> std::result::Result<RefreshToken, E> {
        new_token.validate()?;

        // Find the user by the username
        let user = User::from_credentials(
            ctx.data::<sqlx::PgPool>()?,
            new_token.username,
            new_token.password,
        )
        .await?;

        let token_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect();

        // Create a new refresh token
        Ok(sqlx::query_as!(
            RefreshToken,
            r#"
            INSERT INTO refresh_tokens
            (token_string, expires, user_id, client_address, max_jwt_lifetime)
            VALUES ($1, $2, $3, $4, $5) RETURNING *;
            "#,
            token_string,
            Utc::now() + Duration::seconds(new_token.expires_in_seconds as i64),
            user.id,
            ctx.data::<IpAddr>()?.to_string(),
            new_token.max_jwt_lifetime
        )
        .fetch_one(ctx.data::<sqlx::PgPool>()?)
        .await?)
    }
}

#[cfg(test)]
mod tests {
    /// Create a new refresh token.
    #[tokio::test]
    async fn create() {
        gql_test_user!();

        assert!(gql_test!(
            r#"mutation {
                createRefreshToken(newToken: { username: "test_user", password: "password_of_20_characters" }
                  ) { id }
              }
              "#
        )
        .is_ok());
    }

    /// Try to create a token with too long of a lifetime.
    #[tokio::test]
    async fn create_too_long_lifetime() {
        gql_test_user!();

        assert!(gql_test!(
            r#"mutation {
                createRefreshToken(newToken: { username: "test_user", password: "password_of_20_characters", expiresInSeconds: 2629801 }
                  ) { id }
              }
              "#
        )
        .is_err());
    }

    /// The wrong password should not work.
    #[tokio::test]
    async fn create_wrong_password() {
        gql_test_user!();

        assert!(gql_test!(
            r#"mutation {
                createRefreshToken(newToken: { username: "test_user", password: "wrong_password" }
                  ) { id }
              }
              "#
        )
        .is_err());
    }
}
