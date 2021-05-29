use crate::{models::user::User, res::Res};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use anyhow::Result;
use async_graphql::SimpleObject;
use chrono::prelude::*;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use openssl::{pkey::Private, rsa::Rsa};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use uuid::Uuid;

/// Manages the RSA private and public keys, incoming JWT validation and singing of new JWTs.
#[derive(Clone)]
pub struct JWT {
    /// The private key in PEM format.
    private_key: Vec<u8>,
    /// The public key in a PEM format.
    pub public_key: Vec<u8>,
    /// Based on `self.private_key` to encode tokens.
    encoding: EncodingKey,
    /// Based on `self.private_key` to decode tokens.
    decoding: DecodingKey,
}

impl JWT {
    /// Generate a new instance with a newly generated private key.
    pub fn generate() -> Result<Self> {
        Self::from_pem(Rsa::generate(4096)?)
    }

    /// Retrieve a key from a file and check it's validity.
    pub fn from_pem_file(path: &str) -> Result<Self> {
        let contents = read_to_string(path)?.as_bytes().to_vec();

        // Check that the file content is a valid key
        let rsa_pem = Rsa::private_key_from_pem(&contents)?;

        Self::from_pem(rsa_pem)
    }

    pub fn from_pem(pem: Rsa<Private>) -> Result<Self> {
        let public = pem.public_key_to_pem()?;
        let private = pem.private_key_to_pem()?;

        Ok(JWT {
            encoding: EncodingKey::from_rsa_pem(&private)?,
            decoding: DecodingKey::from_rsa_pem(&public)?,
            public_key: public,
            private_key: private,
        })
    }

    /// Encode claim with the application RSA private key.
    pub fn encode(&self, claims: &Claims) -> Result<String> {
        Ok(encode(
            &Header::new(Algorithm::RS256),
            claims,
            &self.encoding,
        )?)
    }

    /// Decode claim with the application RSA private key.
    /// Also checks expiration.
    pub fn decode(&self, token: &str) -> Result<TokenData<Claims>> {
        Ok(decode(
            token,
            &self.decoding,
            &Validation::new(Algorithm::RS256),
        )?)
    }
}

impl FromRequest for JWT {
    type Error = Res<()>;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.app_data::<Self>() {
            Some(jwt) => ok(jwt.clone()),
            _ => {
                error!("JWT does not exists in app's data!");

                err(Res::<()>::error("No JWT in app's data"))
            }
        }
    }
}

/// JWT token claims that are encoded and decoded.
/// `User` could just be `Uuid`, if the other fields are not needed.
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Claims {
    pub user: User,
    pub parent_token: Uuid,
    iat: usize,
    exp: usize,
}

impl Claims {
    /// Create a new claims
    pub fn new(user: User, exp_secs: usize, parent_token: Uuid) -> Claims {
        let now = Utc::now().timestamp() as usize;

        Claims {
            user,
            parent_token,
            iat: now,
            exp: now + exp_secs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// New `Claims` with an expiration of 5 minutes.
    fn test_claims() -> Claims {
        Claims::new(
            User {
                id: Uuid::new_v4(),
                created: Utc::now(),
                modified: Utc::now(),
                username: "jwt_username".into(),
                email: None,
                display_name: None,
                password_hash: "".into(),
                groups: vec![],
            },
            600,
            Uuid::new_v4(),
        )
    }

    /// Test that valid claims pass the validation.
    #[test]
    fn valid_claims() {
        let claims = test_claims();
        let jwt = JWT::generate().unwrap();

        let encoded = jwt.encode(&claims).unwrap();
        let decoded = jwt.decode(&encoded);

        // Should be valid, since the 10 minute expiration.
        assert!(decoded.is_ok())
    }

    /// Modify the expiration to the past.
    #[test]
    fn expired_claims() {
        let mut claims = test_claims();
        // Edit the timestamp to the past
        claims.exp -= 1000;

        let jwt = JWT::generate().unwrap();

        let encoded = jwt.encode(&claims).unwrap();
        let decoded = jwt.decode(&encoded);

        assert!(!decoded.is_ok())
    }
}
