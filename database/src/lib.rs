use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct DB {
    pool: Pool<Postgres>,
}

impl DB {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DB { pool }
    }

    pub async fn connect(conn_str: &str) -> Result<Self, sqlx::Error> {
        Ok(DB {
            pool: PgPoolOptions::new()
                .acquire_timeout(Duration::from_secs(5))
                .idle_timeout(Some(Duration::from_secs(60)))
                .connect(conn_str)
                .await?,
        })
    }

    pub fn pool(mut self, pool: Pool<Postgres>) -> Self {
        self.pool = pool;
        self
    }

    pub fn set_pool(&mut self, pool: Pool<Postgres>) -> &mut Self {
        self.pool = pool;
        self
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
