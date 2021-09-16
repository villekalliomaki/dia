use crate::{db::RedisConn, Res};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use anyhow::Result;
use futures::future::{err, ok, Ready};
use humantime::format_duration;
use redis::{aio::Connection, AsyncCommands};
use std::{fmt::Display, net::IpAddr, time::Duration};
use uuid::Uuid;

/// Describes a group of rate limiting. Can be used in multiple endpoints.
#[derive(Display)]
pub enum Group {
    General,
    Login,
    Register,
}

/// How the client is identified. Address when a user is not known, and a user when possible.
/// Refresh token is used when wanting to rate limit a single session.
#[derive(Display)]
pub enum Identifier {
    Address(IpAddr),
    User(Uuid),
}

pub struct Limiter {
    pub group: Group,
    pub identifier: Identifier,
    /// How long until key lives in Redis.
    /// In seconds.
    pub bucket_lifetime: Duration,
    /// What the buckets requests left -count is set as when created.
    pub full_count: u64,
}

impl Limiter {
    /// Create a new limiter.
    pub fn new(
        group: Group,
        identifier: Identifier,
        bucket_lifetime: Duration,
        full_count: u64,
    ) -> Limiter {
        Limiter {
            group,
            identifier,
            bucket_lifetime,
            full_count,
        }
    }

    /// New limiter with the given identifier.
    /// Bucket lifetime 1 hour, with 60 requests.
    pub fn default(identifier: Identifier) -> Limiter {
        Limiter {
            group: Group::General,
            identifier,
            bucket_lifetime: Duration::from_secs(60 * 60),
            full_count: 60,
        }
    }

    /// Check if the rate limit is exceeded.
    /// Also counts the request.
    pub async fn check(&self, mut con: Connection) -> Result<()> {
        let id = format!("RL_COUNTER_{}_{}", self.group, self.identifier);

        let exists: bool = con.exists(&id).await?;

        if !exists {
            // There is no counter, create a new one

            // -1 for this request
            con.set(&id, self.full_count - 1).await?;

            // Set the expiration
            con.expire(&id, self.bucket_lifetime.as_secs() as usize)
                .await?;
        } else {
            // A counter exists, check if >= 1, -1
            // Get the value

            let count: usize = con.get(&id).await?;

            if count == 0 {
                // No requests left

                bail!(
                    "You are rate limited! Try again in {}. (In group '{}', identified by {}).",
                    format_duration(Duration::from_secs(con.ttl::<_, u64>(&id).await?)),
                    self.group,
                    self.identifier
                )
            } else {
                // Atleast one request left, take 1 out

                // Have to increase by -1, because .decr doesn't exists with 0.17.0
                con.incr::<_, _, u64>(&id, -1).await?;
            }
        }

        Ok(())
    }

    /// Set the bucket lifetime as seconds.
    pub fn lifetime_seconds(&mut self, seconds: u64) -> &mut Self {
        self.bucket_lifetime = Duration::from_secs(seconds);

        self
    }

    /// Set the default bucket count it starts with.
    pub fn full_count(&mut self, count: u64) -> &mut Self {
        self.full_count = count;

        self
    }

    /// Set to the address to enum variant `Identifier::Address`, with the given address.
    pub fn address(&mut self, addr: IpAddr) -> &mut Self {
        self.identifier = Identifier::Address(addr);

        self
    }

    /// Identifier to `Identifier::User` with the user's UUID.
    pub fn user(&mut self, user: Uuid) -> &mut Self {
        self.identifier = Identifier::User(user);

        self
    }

    // Methods to quicly set the group, without importing the enum.

    /// Set the group to `Group::Register`.
    pub fn register(&mut self) -> &mut Self {
        self.group = Group::Register;

        self
    }

    /// Set the group to `Group::Login`.
    pub fn login(&mut self) -> &mut Self {
        self.group = Group::Login;

        self
    }

    /// Set the group to `Group::General`.
    pub fn general(&mut self) -> &mut Self {
        self.group = Group::General;

        self
    }
}

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

    /// Run a limiter check. Gets a Redis connection.
    pub async fn run(&self, limiter: &Limiter) -> Result<()> {
        let con = self.redis_conn.conn_async().await?;

        limiter.check(con).await?;

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::RedisConn, res::Res, Config, CONF_FILE};
    use actix_web::{test, web, App};

    fn test_limiter() -> Limiter {
        Limiter {
            group: Group::General,
            identifier: Identifier::Address("127.0.0.1".parse().unwrap()),
            bucket_lifetime: Duration::from_secs(60),
            full_count: 10,
        }
    }

    fn redis_conn() -> RedisConn {
        RedisConn::new(&Config::from_file(CONF_FILE))
    }

    async fn handler(rl: RateLimiter) -> Res<()> {
        match rl.run(&test_limiter()).await {
            Ok(_) => Res::ok("", ()),
            Err(_) => Res::<()>::error(""),
        }
    }

    /// Send too many requests. Rate limiter should deny them.
    #[actix_rt::test]
    async fn block_requests() {
        let c = redis_conn();
        let l = RateLimiter::new(c);

        let mut app =
            test::init_service(App::new().app_data(l).route("/", web::get().to(handler))).await;

        // Should be fine because bucket started with 10
        for _ in 0..10 {
            assert!(
                test::call_service(&mut app, test::TestRequest::default().to_request())
                    .await
                    .status()
                    .is_success()
            );
        }

        // Bucket capasity is now 0, should fail
        assert!(
            !test::call_service(&mut app, test::TestRequest::default().to_request())
                .await
                .status()
                .is_success()
        );
    }
}
