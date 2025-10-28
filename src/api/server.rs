use crate::config::AppConfig;
use crate::modules::notes;
// use crate::api::middleware;

use axum::{Router};
use sqlx::SqlitePool;
use tokio::signal;
// use tower::ServiceBuilder;
// use tower_http::cors::{CorsLayer, Any};
use tracing::info;

pub struct Server {
    pool: SqlitePool,
    cfg: AppConfig,
}

impl Server {
    pub fn new(pool: SqlitePool, cfg: AppConfig) -> Self {
        Self { pool, cfg }
    }

    pub async fn run(&self, port: u16) -> anyhow::Result<()> {
        // Combine middleware stack
        // let middleware_stack = ServiceBuilder::new()
        //     .layer(middleware::request_id_layer())
        //     .layer(middleware::real_ip_layer())
        //     .layer(middleware::recover_layer())
        //     .layer(CorsLayer::new()
        //         .allow_origin(Any)
        //         .allow_methods(Any)
        //         .allow_headers(Any))
        //     .into_inner();

        // Build routes and attach state
        let app = Router::new()
            .nest("/api/notes", notes::routes())
            .with_state(self.pool.clone());
            // .layer(middleware::stack());

        // Bind and serve with graceful shutdown
        let addr = format!("{}:{}", self.cfg.app_host, port);
        info!("🌐 Listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        info!("🧹 Server gracefully stopped");
        Ok(())
    }
}

/// Graceful shutdown signal
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("🛑 Shutdown signal received");
}
