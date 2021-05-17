use crate::config::Config;
use sqlx::postgres::{PgPool, PgPoolOptions};

/**
 * Create a Sqlx pool. Only supports PostgreSQL currently. Panics if fails.
 */
pub async fn create_sqlx_pool(conf: &Config) -> PgPool {
    log::info!(
        "Creating SQLX pool with {} connections",
        conf.pg.max_connections
    );

    let pool = PgPoolOptions::new()
        .max_connections(conf.pg.max_connections)
        .connect(&conf.pg.url)
        .await
        .unwrap();

    migrate(&pool).await;

    pool
}

/**
 * Run migrations in the speficied directory. Can panic on failure.
 */
pub async fn migrate(pool: &PgPool) {
    log::info!("Running SQLX migrations");

    sqlx::migrate!("./migrations").run(pool).await.unwrap();
}
