//! Error types for Torznab client operations

use thiserror::Error;

/// Errors that can occur when interacting with Torznab/Newznab indexers
#[derive(Error, Debug)]
pub enum TorznabError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// XML parsing failed
    #[error("XML parsing failed: {0}")]
    XmlParseError(#[from] quick_xml::DeError),

    /// Invalid response format
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// API key is invalid or missing
    #[error("Invalid or missing API key")]
    InvalidApiKey,

    /// Indexer returned an error
    #[error("Indexer error: {code} - {description}")]
    IndexerError {
        code: String,
        description: String,
    },

    /// Capability not supported by indexer
    #[error("Capability not supported: {0}")]
    UnsupportedCapability(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after: {0:?}")]
    RateLimitExceeded(Option<std::time::Duration>),

    /// Timeout occurred
    #[error("Request timeout")]
    Timeout,

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Missing required parameter
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type alias for Torznab operations
pub type Result<T> = std::result::Result<T, TorznabError>;

impl From<TorznabError> for stellarr_core::Error {
    fn from(err: TorznabError) -> Self {
        stellarr_core::Error::ExternalService(err.to_string())
    }
}
