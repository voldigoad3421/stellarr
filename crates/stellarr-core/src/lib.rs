//! Core domain models and business logic for Stellarr media management
//!
//! This crate contains:
//! - Domain models (Media, Series, Episodes, etc.)
//! - Quality profiles and upgrade logic
//! - Indexer and download client configuration
//! - Release scoring and selection algorithms
//! - Business logic traits
//! - Common error types

pub mod domain;
pub mod error;
pub mod traits;

pub use error::{Error, Result};

// Re-export commonly used domain types for convenience
pub use domain::{
    // Common types
    Pagination, PaginatedResponse,
    // Media types
    MediaType, MediaStatus, MediaItem,
    // Series types
    Series, SeriesStatus, Season, SeasonStatus, Episode, EpisodeStatus,
    // Quality types
    Quality, Resolution, QualityProfile, QualityItem,
    // Indexer types
    Indexer, IndexerProtocol, IndexerSearchParams, IndexerStats,
    // Download types
    DownloadClient, DownloadClientType, Download, DownloadStatus,
    // Release types
    Release, ReleaseSelector,
};
