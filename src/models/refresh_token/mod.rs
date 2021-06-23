mod mutation;
mod query;

pub use mutation::RefreshTokenMutation;
pub use query::RefreshTokenQuery;

use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A refresh token is used to generate new JWTs.
#[derive(SimpleObject)]
pub struct RefreshToken {
    /// Identifier used to identify a refresh token without exposing the token string.
    pub id: Uuid,
    /// Identifies the token when generating new JWTs.
    /// Might be hidden in some queries.
    pub token_string: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub expires: DateTime<Utc>,
    pub user_id: Uuid,
    /// The client's address from headers or the socket.
    pub client_address: String,
    /// Maximum valid lifetime for signed JWTs.
    pub max_jwt_lifetime: i32,
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
            token_string: "token_string".into(),
            created: Utc::now(),
            modified: Utc::now(),
            expires: Utc::now(),
            user_id: Uuid::new_v4(),
            client_address: String::new(),
            max_jwt_lifetime: 60,
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
