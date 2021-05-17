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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CONF_FILE;

    async fn test_pool() -> PgPool {
        create_sqlx_pool(&Config::from_file(CONF_FILE)).await
    }

    #[tokio::test]
    /**
     * Test pool creation and connectivity.
     */
    async fn sqlx_pool() {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(1_i64)
            .fetch_one(&test_pool().await)
            .await
            .unwrap();

        assert_eq!(row.0, 1);
    }

    #[tokio::test]
    /**
     * Test running migrations.
     */
    async fn sqlx_migrations() {
        migrate(&test_pool().await).await
    }
}
