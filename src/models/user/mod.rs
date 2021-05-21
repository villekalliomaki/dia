mod mutation;
mod query;

pub use mutation::UserMutation;
pub use query::UserQuery;

use anyhow::{bail, Result};
use async_graphql::*;
use chrono::{DateTime, Utc};
use sodiumoxide::crypto::pwhash::argon2id13;
use uuid::Uuid;

#[derive(SimpleObject, Clone)]
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

    /**
     * Tests the password against current users hash.
     * Returns `true` on a match and `false` if the password is wrong.
     * Usage with `spawn_blocking` is preferred.
     */
    pub fn validate_password<S: Into<String>>(&self, password: S) -> Result<bool> {
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
            Some(hp) => Ok(argon2id13::pwhash_verify(&hp, password.into().as_bytes())),
            _ => bail!("Password was invalid"),
        }
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

        assert_eq!(user.validate_password("a_password").unwrap(), true);
    }

    #[test]
    fn user_password_invalid() {
        let user = test_user();

        assert_eq!(user.validate_password("wrong_password").unwrap(), false);
    }
}
