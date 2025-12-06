//! TV Series domain models
//!
//! This module extends the base media model for TV series, which have
//! hierarchical structure: Series -> Seasons -> Episodes.
//!
//! Design philosophy:
//! - Series inherits from MediaItem conceptually but maintains its own structure
//! - Each level (series, season, episode) can be monitored independently
//! - Episodes track their own download status for granular control

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use super::media::MediaStatus;

/// Series status from metadata provider
///
/// Indicates whether a series is ongoing, completed, or upcoming.
/// This affects monitoring behavior and metadata refresh strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "series_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SeriesStatus {
    /// Series is currently airing new episodes
    Continuing,
    /// Series has concluded, no new episodes expected
    Ended,
    /// Series announced but not yet aired
    Upcoming,
    /// Series canceled before completion
    Canceled,
}

/// TV Series (top-level container)
///
/// Represents a complete TV series. Episodes are organized into seasons.
/// The series itself doesn't have a download status - that's tracked per episode.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Series {
    /// Internal unique identifier
    pub id: Uuid,

    /// The Movie Database (TMDb) identifier
    pub tmdb_id: i64,

    /// TV Database (TVDb) identifier (optional, for compatibility)
    pub tvdb_id: Option<i64>,

    /// Internet Movie Database (IMDb) identifier (optional)
    pub imdb_id: Option<String>,

    /// Display title
    pub title: String,

    /// Original title (in original language)
    pub original_title: Option<String>,

    /// First air year
    pub year: Option<i32>,

    /// Plot synopsis
    pub overview: Option<String>,

    /// Relative path to poster image (on TMDb CDN)
    pub poster_path: Option<String>,

    /// Relative path to backdrop image (on TMDb CDN)
    pub backdrop_path: Option<String>,

    /// Current airing status of the series
    pub series_status: SeriesStatus,

    /// Reference to the quality profile governing this series
    pub quality_profile_id: Uuid,

    /// Root directory path for this series
    /// All seasons/episodes should be under this directory
    pub path: Option<PathBuf>,

    /// Timestamp when this series was added to the library
    pub added_at: DateTime<Utc>,

    /// Timestamp of last metadata refresh
    pub updated_at: DateTime<Utc>,

    /// Whether this series is actively monitored
    /// When false, no episodes will be automatically downloaded
    pub monitored: bool,
}

impl Series {
    /// Create a new series with default values
    pub fn new(tmdb_id: i64, title: String, quality_profile_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tmdb_id,
            tvdb_id: None,
            imdb_id: None,
            title,
            original_title: None,
            year: None,
            overview: None,
            poster_path: None,
            backdrop_path: None,
            series_status: SeriesStatus::Continuing,
            quality_profile_id,
            path: None,
            added_at: now,
            updated_at: now,
            monitored: true,
        }
    }

    /// Check if this series should be actively monitored
    pub fn should_monitor(&self) -> bool {
        self.monitored && !matches!(self.series_status, SeriesStatus::Ended)
    }
}

/// Season status
///
/// Tracks the overall download state of a season.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "season_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SeasonStatus {
    /// No episodes downloaded yet
    Missing,
    /// Some episodes downloaded
    Partial,
    /// All episodes downloaded
    Complete,
    /// All aired episodes downloaded (some may not have aired yet)
    Current,
}

/// Season within a TV series
///
/// Represents a single season. Seasons can be monitored independently of the series.
/// Season 0 typically represents specials/extras.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Season {
    /// Internal unique identifier
    pub id: Uuid,

    /// Parent series identifier
    pub series_id: Uuid,

    /// Season number (0 for specials)
    pub season_number: i32,

    /// Display name (e.g., "Season 1", "Specials")
    pub name: String,

    /// Season overview/description
    pub overview: Option<String>,

    /// First episode air date for this season
    pub air_date: Option<NaiveDate>,

    /// Relative path to season poster
    pub poster_path: Option<String>,

    /// Total number of episodes in this season (from metadata)
    pub episode_count: i32,

    /// Current download status of this season
    pub status: SeasonStatus,

    /// Whether this season is monitored for automatic downloads
    /// Inherits from series but can be overridden
    pub monitored: bool,
}

impl Season {
    /// Create a new season
    pub fn new(series_id: Uuid, season_number: i32, episode_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            series_id,
            season_number,
            name: if season_number == 0 {
                "Specials".to_string()
            } else {
                format!("Season {}", season_number)
            },
            overview: None,
            air_date: None,
            poster_path: None,
            episode_count,
            status: SeasonStatus::Missing,
            monitored: season_number > 0, // Don't monitor specials by default
        }
    }
}

/// Episode status
///
/// More granular than MediaStatus, specific to episode lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "episode_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EpisodeStatus {
    /// Episode has not aired yet
    Unaired,
    /// Episode has aired but not downloaded
    Missing,
    /// Episode is queued or currently downloading
    Downloading,
    /// Episode is downloaded and available
    Downloaded,
    /// Download failed
    Failed,
    /// Episode is available but upgrade is available
    UpgradeAvailable,
}

/// Episode within a season
///
/// The most granular unit in the series hierarchy. Each episode tracks its own
/// download status, quality, and file path.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Episode {
    /// Internal unique identifier
    pub id: Uuid,

    /// Parent season identifier
    pub season_id: Uuid,

    /// Parent series identifier (denormalized for query performance)
    pub series_id: Uuid,

    /// Episode number within the season
    pub episode_number: i32,

    /// Season number (denormalized for convenience)
    pub season_number: i32,

    /// Episode title
    pub title: String,

    /// Episode overview/synopsis
    pub overview: Option<String>,

    /// Original air date
    pub air_date: Option<NaiveDate>,

    /// Episode runtime in minutes
    pub runtime: Option<i32>,

    /// Relative path to episode still/thumbnail
    pub still_path: Option<String>,

    /// Current status of this episode
    pub status: EpisodeStatus,

    /// The quality of the downloaded file (if any)
    pub current_quality: Option<String>,

    /// Filesystem path to the episode file
    pub file_path: Option<PathBuf>,

    /// Whether this episode is monitored for download
    /// Inherits from season/series but can be overridden
    pub monitored: bool,

    /// Timestamp when this episode was added
    pub added_at: DateTime<Utc>,

    /// Timestamp of last update
    pub updated_at: DateTime<Utc>,
}

impl Episode {
    /// Create a new episode
    pub fn new(
        series_id: Uuid,
        season_id: Uuid,
        season_number: i32,
        episode_number: i32,
        title: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            season_id,
            series_id,
            episode_number,
            season_number,
            title,
            overview: None,
            air_date: None,
            runtime: None,
            still_path: None,
            status: EpisodeStatus::Unaired,
            current_quality: None,
            file_path: None,
            monitored: true,
            added_at: now,
            updated_at: now,
        }
    }

    /// Check if this episode has aired
    pub fn has_aired(&self) -> bool {
        self.air_date
            .map(|date| date <= Utc::now().date_naive())
            .unwrap_or(false)
    }

    /// Check if this episode is available on disk
    pub fn is_available(&self) -> bool {
        matches!(
            self.status,
            EpisodeStatus::Downloaded | EpisodeStatus::UpgradeAvailable
        ) && self.file_path.is_some()
    }

    /// Check if this episode needs to be downloaded
    pub fn needs_download(&self) -> bool {
        self.monitored
            && self.has_aired()
            && matches!(self.status, EpisodeStatus::Missing | EpisodeStatus::Failed)
    }

    /// Check if this episode is eligible for upgrade
    pub fn can_upgrade(&self) -> bool {
        self.monitored && self.status == EpisodeStatus::UpgradeAvailable
    }

    /// Mark this episode as downloaded
    pub fn mark_downloaded(&mut self, quality: String, file_path: PathBuf) {
        self.current_quality = Some(quality);
        self.file_path = Some(file_path);
        self.status = EpisodeStatus::Downloaded;
        self.updated_at = Utc::now();
    }

    /// Update the status
    pub fn set_status(&mut self, status: EpisodeStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Get a display identifier (e.g., "S01E05")
    pub fn episode_code(&self) -> String {
        format!("S{:02}E{:02}", self.season_number, self.episode_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_episode_code() {
        let episode = Episode::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            1,
            5,
            "Test Episode".to_string(),
        );
        assert_eq!(episode.episode_code(), "S01E05");
    }

    #[test]
    fn test_episode_aired() {
        let mut episode = Episode::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            1,
            1,
            "Test".to_string(),
        );

        // No air date
        assert!(!episode.has_aired());

        // Future air date
        episode.air_date = Some(Utc::now().date_naive() + chrono::Duration::days(7));
        assert!(!episode.has_aired());

        // Past air date
        episode.air_date = Some(Utc::now().date_naive() - chrono::Duration::days(7));
        assert!(episode.has_aired());
    }

    #[test]
    fn test_season_specials() {
        let season = Season::new(Uuid::new_v4(), 0, 5);
        assert_eq!(season.name, "Specials");
        assert!(!season.monitored); // Specials not monitored by default
    }
}
