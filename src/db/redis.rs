use crate::config::Config;
use redis::Client;

/**
 * Create a Redis client. Can panic.
 */
pub fn create_redis_client(conf: &Config) -> Client {
    log::info!("Creating Redis client");

    Client::open(&conf.rd.url[..]).unwrap()
}
