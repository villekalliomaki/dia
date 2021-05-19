use async_graphql::*;
use chrono::{DateTime, Utc};
use sodiumoxide::crypto::pwhash::argon2id13;
use std::time::Instant;
use tokio::task::spawn_blocking;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct User {
    id: Uuid,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    username: String,
    email: Option<String>,
    display_name: Option<String>,
    #[graphql(skip)]
    password_hash: String,
    groups: Vec<String>,
}

impl User {
    /**
     * Hash the given password with argon2id13.
     * If used in an asyncronous context, `spawn_blocking` should be used,
     * since task is CPU intensive.
     */
    pub fn hash_password<S: Into<String>>(new_password: S) -> Result<String, Error> {
        sodiumoxide::init().unwrap();

        let hashed = match argon2id13::pwhash(
            new_password.into().as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(h) => h,
            Err(()) => return Err(Error::new("Failed to hash password.")),
        };

        match std::str::from_utf8(&hashed.0) {
            Ok(s) => Ok(s.trim_end_matches('\u{0}').to_string()),
            Err(err) => Err(Error::new(format!("Failed to convert to utf-8: {}", err))),
        }
    }

    /**
     * Tests the password against current users hash.
     * Returns `true` on a match and `false` if the password is wrong.
     */
    pub fn validate_password<S: Into<String>>(&self, password: S) -> bool {
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
            Some(hp) => argon2id13::pwhash_verify(&hp, password.into().as_bytes()),
            _ => false,
        }
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn test_user(&self, ctx: &Context<'_>) -> Result<User> {
        let sqlx = ctx.data::<sqlx::PgConnection>()?;

        Ok(User {
            id: uuid::Uuid::new_v4(),
            created: Utc::now(),
            modified: Utc::now(),
            username: "username".to_string(),
            email: None,
            display_name: None,
            password_hash: "password".to_string(),
            groups: vec!["users".to_string()],
        })
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

        assert_eq!(user.validate_password("a_password"), true);
    }

    #[test]
    fn user_password_invalid() {
        let user = test_user();

        assert_eq!(user.validate_password("wrong_password"), false);
    }
}
