use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

/// API error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// API error type
#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    DatabaseError(String),
    TmdbError(String),
    ValidationError(String),
    Conflict(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::TmdbError(msg) => write!(f, "TMDB error: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            ApiError::NotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND"),
            ApiError::BadRequest(_) => (actix_web::http::StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            ApiError::ValidationError(_) => (actix_web::http::StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            ApiError::Conflict(_) => (actix_web::http::StatusCode::CONFLICT, "CONFLICT"),
            ApiError::InternalError(_) | ApiError::DatabaseError(_) | ApiError::TmdbError(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
            ),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            details: None,
        })
    }
}

// Convert from stellarr-core errors
impl From<stellarr_core::Error> for ApiError {
    fn from(err: stellarr_core::Error) -> Self {
        match err {
            stellarr_core::Error::NotFound(msg) => ApiError::NotFound(msg),
            stellarr_core::Error::InvalidInput(msg) => ApiError::BadRequest(msg),
            _ => ApiError::InternalError(err.to_string()),
        }
    }
}

// Convert from database errors
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                // Check for unique constraint violations
                if let Some(code) = db_err.code() {
                    if code == "23505" || code == "2067" { // PostgreSQL and SQLite unique violation
                        return ApiError::Conflict("Resource already exists".to_string());
                    }
                }
                ApiError::DatabaseError(db_err.to_string())
            }
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

// Convert from TMDB errors
impl From<stellarr_providers::tmdb::error::TmdbError> for ApiError {
    fn from(err: stellarr_providers::tmdb::error::TmdbError) -> Self {
        use stellarr_providers::tmdb::error::TmdbError;
        match err {
            TmdbError::NotFound(_) => ApiError::NotFound("TMDB resource not found".to_string()),
            TmdbError::Authentication => ApiError::InternalError("TMDB authentication failed".to_string()),
            TmdbError::RateLimit => ApiError::InternalError("TMDB rate limit exceeded".to_string()),
            _ => ApiError::TmdbError(err.to_string()),
        }
    }
}

// Convert from anyhow errors
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::InternalError(err.to_string())
    }
}

// Convert from UUID parsing errors
impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> Self {
        ApiError::BadRequest(format!("Invalid UUID: {}", err))
    }
}
