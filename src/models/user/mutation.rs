use super::{regex, User};
use crate::Config;
use async_graphql::*;
use tokio::task::spawn_blocking;
use validator::Validate;

/// A new user with an optional email address.
#[derive(Validate, InputObject, Clone)]
struct NewUser {
    #[validate(regex = "regex::USERNAME")]
    username: String,
    #[validate(email)]
    email: Option<String>,
    #[validate(regex = "regex::PASSWORD")]
    password: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Create a new user if registerations are allowed. Rate limited to 5 tries per hour.
    async fn create_user(&self, ctx: &Context<'_>, new_user: NewUser) -> Result<User> {
        if !ctx.data::<Config>()?.allow_registerations {
            return Err(Error::new("Registerations not allowed"));
        }

        // To not interfere with tests.
        // Rate limiting only in release builds.
        #[cfg(not(test))]
        {
            use crate::access::{Identifier, Limiter, RateLimiter};
            use std::net::IpAddr;

            ctx.data::<RateLimiter>()?
                .run(
                    &Limiter::default(Identifier::Address(ctx.data::<IpAddr>()?.clone()))
                        .register()
                        .lifetime_seconds(60 * 60)
                        .full_count(5),
                )
                .await?;
        }

        new_user.validate()?;

        let sqlx = ctx.data::<sqlx::PgPool>()?;

        let c = new_user.clone();
        let hashed_password = spawn_blocking(|| User::hash_password(c.password)).await??;

        Ok(sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *;",
            new_user.username,
            new_user.email,
            hashed_password
        )
        .fetch_one(sqlx)
        .await?)
    }
}

#[cfg(test)]
mod tests {
    /// Creating users should be impossible when `allow_registerations = true` in `config.toml`.
    #[tokio::test]
    async fn disable_registerations() {
        // Can't use the macro since Config has to be altered.

        use crate::{gql::build_schema, Config, CONF_FILE};
        use async_graphql::Request;

        let mut conf = Config::from_file(CONF_FILE);
        conf.allow_registerations = false;

        let mut req = Request::new(
            r#"mutation {
                createUser(newUser: { username: "username", password: "password" }) {
                  id
                }
              }
              "#,
        );

        req = req.data(conf);

        let res = build_schema().execute(req).await;

        assert!(res.is_err());
    }

    /// Successfully create a new user. Might fail if not using a clean database instance.
    #[tokio::test]
    async fn create_user() {
        assert!(gql_test!(
            r#"mutation {
                createUser(newUser: { username: "username", password: "password_of_20_characters", email: "test_username@email.com" }) {
                  id
                }
              }
              "#
        )
        .is_ok());
    }
}
