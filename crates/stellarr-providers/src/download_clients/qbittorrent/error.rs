use thiserror::Error;

#[derive(Debug, Error)]
pub enum QBittorrentError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Torrent not found: {0}")]
    TorrentNotFound(String),

    #[error("Failed to add torrent: {0}")]
    AddTorrentFailed(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Session expired or not authenticated")]
    SessionExpired,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, QBittorrentError>;
