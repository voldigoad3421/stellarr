use stellarr_db::Database;
use stellarr_providers::TmdbClient;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: Database,

    /// TMDB client for metadata
    pub tmdb: Arc<TmdbClient>,
}

impl AppState {
    /// Create a new application state
    pub fn new(db: Database, tmdb: TmdbClient) -> Self {
        Self {
            db,
            tmdb: Arc::new(tmdb),
        }
    }
}
