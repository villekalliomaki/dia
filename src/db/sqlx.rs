use crate::config::Config;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Error,
};

/**
 * Create a Sqlx pool. Only supports PostgreSQL currently.
 */
pub async fn create_sqlx_pool(conf: &Config) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(conf.pg.max_connections)
        .connect(&conf.pg.url)
        .await
}
