mod config;
mod cli;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;
use crate::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration
    let config = AppConfig::load()?;

    tracing::info!("Starting Stellarr v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Configuration loaded: {:?}", config);

    // Initialize database
    let db = stellarr_db::Database::connect(&config.database).await?;
    tracing::info!("Database connection established");

    // Run migrations
    db.migrate().await?;
    tracing::info!("Database migrations completed");

    // Create application state
    let tmdb = stellarr_providers::TmdbClient::new(config.api_keys.tmdb().to_string())?;
    let state = stellarr_api::AppState::new(db, tmdb);

    // Start API server
    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting API server on {}", bind_address);

    stellarr_api::start_server(state, &bind_address).await?;

    Ok(())
}
