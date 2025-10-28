use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;
use crate::{config::AppConfig, error::AppErr};

/// Initialize database with connection options and schema setup.
pub async fn init_db(cfg: &AppConfig) -> Result<SqlitePool, AppErr> {
    // Ensure database directory exists
    if let Some(path) = cfg.database_url.strip_prefix("sqlite://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent).expect("Failed to create database directory");
        }
    }

    // Setup connection pool with fixed performance options
    let pool = SqlitePoolOptions::new()
        .max_connections(10)              // Up to 10 concurrent connections
        .min_connections(2)               // Keep 2 warm
        .test_before_acquire(true)        // Validate connections before use
        .connect(&cfg.database_url)
        .await?;

    // Initialize schema if not exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
