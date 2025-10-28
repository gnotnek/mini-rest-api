use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::io;

/// Initialize global logger with console + file output.
pub fn init_logger() {
    // You can change the rotation to HOURLY or DAILY as needed
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");

    // Combine environment filter + formatting layer
    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug,sqlx=warn"));

    let fmt_layer = fmt::layer()
        .with_target(false) // don’t print module path
        .with_thread_ids(false)
        .with_ansi(true) // colored output in terminal
        .with_writer(io::stdout);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(file_appender);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(file_layer)
        .init();

    tracing::info!("📜 Logger initialized");
}
