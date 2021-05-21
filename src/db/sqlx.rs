use crate::config::Config;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use sqlx::postgres::{PgPool, PgPoolOptions};

/**
 * Sqlx pool wrapper, since you can't implement traits on foreign structs.
 */
#[derive(Clone)]
pub struct SqlxConn(PgPool);

impl SqlxConn {
    /**
     * Create a pool from a config. Can panic.
     */
    pub async fn new(conf: &Config) -> Self {
        log::info!(
            "Creating SQLX pool with {} connections",
            conf.pg.max_connections
        );

        let pool = PgPoolOptions::new()
            .max_connections(conf.pg.max_connections)
            .connect(&conf.pg.url)
            .await
            .unwrap();

        SqlxConn(pool)
    }

    /**
     * Run migrations from ./migrations, panics on failure.
     */
    pub async fn migrate(&self) {
        log::info!("Running SQLX migrations");

        sqlx::migrate!("./migrations").run(&self.0).await.unwrap();
    }

    pub fn into_inner(self) -> PgPool {
        self.0
    }
}

impl FromRequest for SqlxConn {
    type Error = ();
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.app_data::<SqlxConn>() {
            Some(conn) => ok(conn.clone()),
            _ => {
                log::error!("SqlxConn does not exists in app's data!");

                err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, CONF_FILE};

    #[tokio::test]
    /**
     * Mainly for testing the configuration.
     */
    async fn create_sqlx_conn() {
        let conf = &Config::from_file(CONF_FILE);

        SqlxConn::new(conf).await;
    }

    #[tokio::test]
    /**
     * Test migrations.
     */
    async fn sqlx_migrations() {
        let conf = &Config::from_file(CONF_FILE);

        SqlxConn::new(conf).await.migrate().await;
    }
}
