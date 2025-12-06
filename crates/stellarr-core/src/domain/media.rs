//! Core media domain models
//!
//! This module defines the fundamental media types used throughout Stellarr.
//! It provides a unified abstraction for both movies and series.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Media type discriminator
///
/// Distinguishes between different types of media content in the system.
/// This is used for routing, display, and provider-specific logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// Feature film or movie
    Movie,
    /// TV series or show
    Series,
}

/// Media availability and download status
///
/// Represents the current state of a media item in the download lifecycle.
/// This enum tracks everything from initial request through to upgrade scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "media_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MediaStatus {
    /// Media is not yet available and not being downloaded
    Missing,
    /// Media is currently being downloaded
    Downloading,
    /// Media is available on disk at the expected quality
    Available,
    /// Download failed or media file is corrupted
    Failed,
    /// Media is available but a higher quality version is available
    UpgradeAvailable,
}

/// Core media item structure
///
/// This represents a single media item (movie or series at the top level).
/// For series, individual episodes have their own status tracking (see series.rs).
///
/// Design decisions:
/// - `tmdb_id` is required as our primary metadata source
/// - `imdb_id` is optional but useful for cross-referencing
/// - `path` uses PathBuf for type-safe filesystem operations
/// - `quality_profile_id` links to quality preferences
/// - `current_quality` tracks what we actually have
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaItem {
    /// Internal unique identifier
    pub id: Uuid,

    /// The Movie Database (TMDb) identifier
    pub tmdb_id: i64,

    /// Internet Movie Database (IMDb) identifier (optional)
    pub imdb_id: Option<String>,

    /// Display title
    pub title: String,

    /// Release year (for movies) or first air year (for series)
    pub year: Option<i32>,

    /// Plot synopsis or overview
    pub overview: Option<String>,

    /// Relative path to poster image (on TMDb CDN)
    pub poster_path: Option<String>,

    /// Relative path to backdrop image (on TMDb CDN)
    pub backdrop_path: Option<String>,

    /// Type of media (movie or series)
    pub media_type: MediaType,

    /// Current download/availability status
    pub status: MediaStatus,

    /// Reference to the quality profile governing this item
    pub quality_profile_id: Uuid,

    /// The quality of the currently downloaded file (if any)
    /// Stored as string to allow flexibility in quality representation
    pub current_quality: Option<String>,

    /// Filesystem path to the media file or directory
    /// For movies: path to the file
    /// For series: path to the series directory
    pub path: Option<PathBuf>,

    /// Timestamp when this item was added to the library
    pub added_at: DateTime<Utc>,

    /// Timestamp of last metadata or status update
    pub updated_at: DateTime<Utc>,

    /// Whether this item is actively monitored for downloads/upgrades
    pub monitored: bool,
}

impl MediaItem {
    /// Create a new media item with default values
    pub fn new(
        tmdb_id: i64,
        title: String,
        media_type: MediaType,
        quality_profile_id: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tmdb_id,
            imdb_id: None,
            title,
            year: None,
            overview: None,
            poster_path: None,
            backdrop_path: None,
            media_type,
            status: MediaStatus::Missing,
            quality_profile_id,
            current_quality: None,
            path: None,
            added_at: now,
            updated_at: now,
            monitored: true,
        }
    }

    /// Check if this media item is available on disk
    pub fn is_available(&self) -> bool {
        matches!(self.status, MediaStatus::Available | MediaStatus::UpgradeAvailable)
            && self.path.is_some()
    }

    /// Check if this media needs to be downloaded
    pub fn needs_download(&self) -> bool {
        self.monitored && matches!(self.status, MediaStatus::Missing | MediaStatus::Failed)
    }

    /// Check if this media is eligible for upgrade
    pub fn can_upgrade(&self) -> bool {
        self.monitored && self.status == MediaStatus::UpgradeAvailable
    }

    /// Update the status and timestamp
    pub fn set_status(&mut self, status: MediaStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Mark this item as downloaded with the given quality and path
    pub fn mark_downloaded(&mut self, quality: String, path: PathBuf) {
        self.current_quality = Some(quality);
        self.path = Some(path);
        self.status = MediaStatus::Available;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_item_creation() {
        let item = MediaItem::new(
            12345,
            "Test Movie".to_string(),
            MediaType::Movie,
            Uuid::new_v4(),
        );

        assert_eq!(item.tmdb_id, 12345);
        assert_eq!(item.title, "Test Movie");
        assert_eq!(item.media_type, MediaType::Movie);
        assert_eq!(item.status, MediaStatus::Missing);
        assert!(item.monitored);
    }

    #[test]
    fn test_media_item_availability() {
        let mut item = MediaItem::new(
            12345,
            "Test Movie".to_string(),
            MediaType::Movie,
            Uuid::new_v4(),
        );

        assert!(!item.is_available());
        assert!(item.needs_download());

        item.mark_downloaded(
            "1080p".to_string(),
            PathBuf::from("/media/Test Movie (2023).mkv"),
        );

        assert!(item.is_available());
        assert!(!item.needs_download());
        assert_eq!(item.status, MediaStatus::Available);
    }
}
