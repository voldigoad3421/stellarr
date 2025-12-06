/// Domain models for Stellarr media management system
///
/// This module contains all core domain types representing the business entities
/// and value objects used throughout the application. The domain is designed to be:
/// - Database-agnostic (though sqlx derives are included for convenience)
/// - Serialization-friendly for API responses
/// - Type-safe with strong enum discriminants
/// - Self-documenting with comprehensive doc comments

pub mod common;
pub mod download;
pub mod indexer;
pub mod media;
pub mod quality;
pub mod release;
pub mod series;

// Re-export common types for convenience
pub use common::*;
pub use download::*;
pub use indexer::*;
pub use media::*;
pub use quality::*;
pub use release::*;
pub use series::*;
