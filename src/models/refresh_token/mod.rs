use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A refresh token is used to generate new JWTs.
#[derive(SimpleObject)]
pub struct RefreshToken {
    id: Uuid,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    expires: DateTime<Utc>,
    user: Uuid,
    client_address: String,
}

impl RefreshToken {
    pub fn is_valid(&self) -> bool {
        self.expires > Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn test_token() -> RefreshToken {
        RefreshToken {
            id: Uuid::new_v4(),
            created: Utc::now(),
            modified: Utc::now(),
            expires: Utc::now(),
            user: Uuid::new_v4(),
            client_address: String::new(),
        }
    }

    #[test]
    fn refresh_token_invalid() {
        let mut token = test_token();

        // Set expiration into past
        token.expires = token.expires - Duration::hours(1);

        assert!(!token.is_valid())
    }

    #[test]
    fn refresh_token_valid() {
        let mut token = test_token();

        // Set expiration into future
        token.expires = token.expires + Duration::hours(1);

        assert!(token.is_valid())
    }
}