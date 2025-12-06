//! Repository module for database operations
//!
//! This module contains all repository implementations for database entities.
//! Repositories provide a clean abstraction over SQL queries and handle
//! conversion between database rows and domain models.

pub mod media;
pub mod series;
pub mod indexer;
pub mod download;
pub mod quality;

pub use media::MediaRepository;
pub use series::SeriesRepository;
pub use indexer::IndexerRepository;
pub use download::DownloadRepository;
pub use quality::QualityProfileRepository;
