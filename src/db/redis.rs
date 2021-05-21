use crate::config::Config;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use redis::{aio, Client, Connection, RedisError};

/**
 * Wrapper for the redis client.
 */
#[derive(Clone)]
pub struct RedisConn(Client);

impl RedisConn {
    /**
     * Create a new client, can panic.
     */
    pub fn new(conf: &Config) -> Self {
        log::info!("Creating Redis client");

        RedisConn(Client::open(conf.rd.url.as_str()).unwrap())
    }

    pub fn into_inner(self) -> Client {
        self.0
    }

    /**
     * Create a normal syncronous connection. Safe to use in handlers and such.
     */
    #[allow(dead_code)]
    pub fn conn(&self) -> Result<Connection, RedisError> {
        self.0.get_connection()
    }

    /**
     * Get an asyncronous connection safely.
     */
    #[allow(dead_code)]
    pub async fn conn_async(&self) -> Result<aio::Connection, RedisError> {
        self.0.get_async_connection().await
    }
}

impl FromRequest for RedisConn {
    type Error = ();
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.app_data::<RedisConn>() {
            Some(conn) => ok(conn.clone()),
            _ => {
                log::error!("RedisConn does not exists in app's data!");

                err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CONF_FILE;
    use redis::AsyncCommands;

    #[tokio::test]
    /**
     * Test creating and using a redis client. Mainly for testing the configuration.
     */
    async fn redis_client() {
        let client = RedisConn::new(&Config::from_file(CONF_FILE));

        let mut conn = client.conn_async().await.unwrap();

        let _: () = conn.set("key", "value").await.unwrap();
    }
}
