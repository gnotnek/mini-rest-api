use crate::api::server::Server;
use crate::db;
use crate::config;
use crate::logger;

use tracing::info;
use config::AppConfig;


pub async fn run() -> anyhow::Result<()> {
    // Init logging
    // Initialize
    let config = AppConfig::from_env();
    logger::init_logger();

    // Database connection
    let pool = db::init_db(&config).await?;

    // Build API server
    let server = Server::new(pool, config);

    // Run server with graceful shutdown
    info!("🚀 Starting API server...");
    server.run(8080).await?;

    Ok(())
}
