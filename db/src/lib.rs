use anyhow::Result;
use std::env;

mod models;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Debug, Clone)]
pub struct Store {
    pub pool: PgPool,
}

impl Store {
    pub async fn new() -> Result<Self> {
        let db_url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        Ok(Self { pool })
    }
}
