use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Core error types for Stellarr
#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Duplicate entry: {0}")]
    Duplicate(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
