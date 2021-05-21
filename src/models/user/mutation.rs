use super::User;
use async_graphql::*;

/// A new user with an optional email address.
#[derive(InputObject)]
struct NewUser {
    username: String,
    email: Option<String>,
    password: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, new_user: NewUser) -> Result<User> {
        let sqlx = ctx.data::<sqlx::PgPool>()?;

        Ok(sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *;",
            new_user.username,
            new_user.email,
            User::hash_password(new_user.password.as_str())?
        )
        .fetch_one(sqlx)
        .await?)
    }
}
