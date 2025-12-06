use thiserror::Error;

/// Errors that can occur when interacting with the TMDB API
#[derive(Debug, Error)]
pub enum TmdbError {
    /// Network error when making HTTP requests
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON deserialization error
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// Rate limit exceeded (HTTP 429)
    #[error("Rate limit exceeded. Please try again later")]
    RateLimit,

    /// Authentication failed (HTTP 401)
    #[error("Authentication failed. Check your access token")]
    Authentication,

    /// Resource not found (HTTP 404)
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// General API error
    #[error("TMDB API error (status {status}): {message}")]
    Api { status: u16, message: String },
}

/// Result type alias for TMDB operations
pub type Result<T> = std::result::Result<T, TmdbError>;
