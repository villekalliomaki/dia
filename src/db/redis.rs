use crate::config::Config;
use redis::Client;

/**
 * Create a Redis client. Can panic.
 */
pub fn create_redis_client(conf: &Config) -> Client {
    log::info!("Creating Redis client");

    Client::open(&conf.rd.url[..]).unwrap()
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
        let client = create_redis_client(&Config::from_file(CONF_FILE));

        let mut conn = client.get_async_connection().await.unwrap();

        let _: () = conn.set("key", "value").await.unwrap();
    }
}
