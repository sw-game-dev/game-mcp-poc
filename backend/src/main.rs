mod api;
mod db;
mod game;
mod mcp;

use std::env;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt().with_env_filter(filter).with_target(false).init();

    tracing::info!("Starting Game REST API Server");

    // Get configuration from environment
    let db_path = env::var("GAME_DB_PATH").unwrap_or_else(|_| "game.db".to_string());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(7397);

    // Start the server
    api::server::start_server(&db_path, port).await?;

    Ok(())
}
