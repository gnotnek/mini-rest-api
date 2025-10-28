mod config;
mod db;
mod error;
mod logger;
mod modules;

use axum::Router;
use modules::notes;
use tracing::info;
use config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize
    let config = AppConfig::from_env();
    logger::init_logger();

    // Initialize database
    let pool = db::init_db(&config).await?;

    // Build routes
    let app = Router::new()
        .nest("/notes", notes::routes())
        .with_state(pool);

    let addr = config.addr();
    info!("🚀 Server running at http://{}", addr);

    // Start server
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app,
    )
    .await?;

    Ok(())
}
