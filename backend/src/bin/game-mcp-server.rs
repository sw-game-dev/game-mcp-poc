use backend::mcp::server::McpServer;
use std::env;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr) // Write logs to stderr, not stdout (which is for JSON-RPC)
        .init();

    tracing::info!("Starting Game MCP Server");

    // Get database path from environment or use default
    let db_path = env::var("GAME_DB_PATH")
        .unwrap_or_else(|_| "game.db".to_string());

    tracing::info!("Using database path: {}", db_path);

    // Create and run server
    match McpServer::new(&db_path) {
        Ok(mut server) => {
            tracing::info!("MCP Server initialized successfully");
            tracing::info!("Listening for JSON-RPC 2.0 requests on stdin...");

            if let Err(e) = server.run() {
                tracing::error!("Server error: {}", e);
                std::process::exit(1);
            }

            tracing::info!("MCP Server shutting down");
        }
        Err(e) => {
            tracing::error!("Failed to initialize MCP Server: {}", e);
            std::process::exit(1);
        }
    }
}
