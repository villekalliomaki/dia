use super::User;

use crate::{
    access::{Identifier, Limiter, RateLimiter},
    Config,
};

use async_graphql::*;
use std::net::IpAddr;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get the user with the correct credentials. Rate limited to 10 tried per hour.
    async fn user(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        // Limit rate to 10 per hour for every address.
        ctx.data::<RateLimiter>()?
            .run(
                &Limiter::default(Identifier::Address(ctx.data::<IpAddr>()?.clone()))
                    .login()
                    .lifetime_seconds(60 * 60)
                    .full_count(10),
            )
            .await?;

        // Convert from one result type to another
        Ok(User::from_credentials(ctx.data::<sqlx::PgPool>()?, username, password).await?)
    }

    /// `true` if the creation of new users is enabled.
    async fn registerations_allowed(&self, ctx: &Context<'_>) -> Result<bool> {
        Ok(ctx.data::<Config>()?.allow_registerations)
    }
}

#[cfg(test)]
mod tests {
    /// Try get user data with the wrong password.
    #[tokio::test]
    async fn user_data_wrong_password() {
        gql_test_user!();

        assert!(gql_test!(
            r#"query {
                user(username: "test_user", password: "wrong_password") { id }
            }
            "#
        )
        .is_err());
    }

    /// Get user data with the right password.
    #[tokio::test]
    async fn user_data_right_password() {
        gql_test_user!();

        assert!(gql_test!(
            r#"query {
                user(username: "test_user", password: "password_of_20_characters") { id }
            }
            "#
        )
        .is_ok());
    }
}
