mod mutation;
mod query;
mod regex;

pub use mutation::UserMutation;
pub use query::UserQuery;

use anyhow::Result;
use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::pwhash::argon2id13;
use sqlx::PgPool;
use tokio::task::spawn_blocking;
use uuid::Uuid;

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub username: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    #[graphql(skip)]
    pub password_hash: String,
    pub groups: Vec<String>,
}

impl User {
    /// Hash the given password with argon2id13.
    /// If used in an asyncronous context, `spawn_blocking` should be used,
    /// since task is CPU intensive.
    pub fn hash_password<S: Into<String>>(new_password: S) -> Result<String> {
        sodiumoxide::init().unwrap();

        let hashed = match argon2id13::pwhash(
            new_password.into().as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(h) => h,
            Err(()) => bail!("Failed to hash password."),
        };

        Ok(std::str::from_utf8(&hashed.0)?
            .trim_end_matches('\u{0}')
            .to_string())
    }

    /// Tests the password against current users hash.
    /// Returns `true` on a match and `false` if the password is wrong.
    /// Usage with `spawn_blocking` is preferred.
    pub fn validate_password<S: Into<String>>(&self, password: S) -> Result<()> {
        sodiumoxide::init().unwrap();

        let mut hash_padded = [0u8; 128];
        self.password_hash
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, val)| {
                hash_padded[i] = val.clone();
            });

        match argon2id13::HashedPassword::from_slice(&hash_padded) {
            Some(hp) => {
                // If password is incorrect, return an error
                if !argon2id13::pwhash_verify(&hp, password.into().as_bytes()) {
                    bail!("Wrong password")
                }
                Ok(())
            }
            _ => bail!("Password "),
        }
    }

    /// Find an user by their username and validate that their password is correct.
    pub async fn from_credentials(
        pool: &PgPool,
        username: String,
        password: String,
    ) -> Result<User> {
        // Find by username
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", &username)
            .fetch_one(pool)
            .await?;

        // Validate the password is correct
        let c = user.clone();
        let password = password.clone();

        // Send to an another thread
        spawn_blocking(move || return c.validate_password(password)).await??;

        // If there is no error, password is correct
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_user() -> User {
        User {
            id: uuid::Uuid::new_v4(),
            created: Utc::now(),
            modified: Utc::now(),
            username: "".to_string(),
            email: None,
            display_name: None,
            password_hash: User::hash_password("a_password").unwrap(),
            groups: vec![],
        }
    }

    #[test]
    fn user_password_valid() {
        let user = test_user();

        assert!(user.validate_password("a_password").is_ok());
    }

    #[test]
    fn user_password_invalid() {
        let user = test_user();

        assert!(user.validate_password("wrong_password").is_err());
    }
}
