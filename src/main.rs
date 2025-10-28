mod cmd;
mod api;
mod modules;
mod error;
mod logger;
mod config;
mod db;

use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("api") => cmd::api::run().await?,
        Some("worker") => {
            println!("🚧 Worker service not implemented yet");
        }
        _ => {
            eprintln!("Usage: cargo run -- <api|worker>");
        }
    }

    Ok(())
}
