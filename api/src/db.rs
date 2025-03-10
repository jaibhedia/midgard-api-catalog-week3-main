use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use thiserror::Error;

pub mod insertions;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Env var error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("sqlx error: {0}")]
    Connection(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

pub async fn init_db() -> Result<PgPool, DbError> {
    let db_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
