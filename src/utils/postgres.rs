use anyhow::Result;
use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};

/// Creates a single Postgres Pool with sqlx
pub async fn create_client(connection_string: &str) -> Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .connect(connection_string)
        .await
        .map_err(anyhow::Error::from)
}
