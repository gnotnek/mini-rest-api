use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("Invalid APP_PORT"),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://storage/notes.db".to_string()),
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }
}
