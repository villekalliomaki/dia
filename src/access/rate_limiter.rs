use crate::{db::RedisConn, models::user::User, Res};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use anyhow::Result;
use futures::future::{err, ok, Ready};
use std::net::IpAddr;

/// Describes a group of rate limiting. Can be used in multiple endpoints.
pub enum Group {
    General,
    Login,
    Register,
}

/// How the client is identified. Address when a user is not known, and a user when possible.
/// Refresh token is used when wanting to rate limit a single session.
pub enum Identifier {
    Address(IpAddr),
    User(User),
}

pub struct Limiter {
    pub group: Group,
    pub identifier: Identifier,
    /// How long until key lives in Redis.
    pub bucket_lifetime: usize,
    /// What the buckets requests left -count is set as when created.
    pub full_count: usize,
}

impl Limiter {}

/// Global rate limiter keeping track of client request rates per IP address.
/// Different resources are in different rate limiting buckets.
#[derive(Clone)]
pub struct RateLimiter {
    redis_conn: RedisConn,
}

impl RateLimiter {
    pub fn new(redis_conn: RedisConn) -> Self {
        RateLimiter { redis_conn }
    }
}

impl FromRequest for RateLimiter {
    type Error = Res<()>;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.app_data::<RateLimiter>() {
            Some(rate_limiter) => ok(rate_limiter.clone()),
            _ => {
                error!("RateLimiter does not exists in app's data!");

                err(Res::<()>::error("No RateLimiter in app's data"))
            }
        }
    }
}
