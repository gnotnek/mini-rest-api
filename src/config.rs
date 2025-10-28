use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app_host: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://storage/notes.db".to_string()),
        }
    }
}
