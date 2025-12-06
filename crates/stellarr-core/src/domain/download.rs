//! Download client configuration and download tracking
//!
//! This module handles:
//! - Download client configuration (qBittorrent, SABnzbd, etc.)
//! - Active download tracking
//! - Download status management
//!
//! Downloads are sent to clients, which then report back status.
//! The system needs to track which downloads are in progress and
//! their current state.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use super::media::MediaType;

/// Download client type/protocol
///
/// Each client type has its own API and capabilities.
/// The client type determines how we communicate with the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "download_client_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DownloadClientType {
    /// qBittorrent torrent client
    QBittorrent,

    /// Transmission torrent client
    Transmission,

    /// Deluge torrent client
    Deluge,

    /// rTorrent/ruTorrent client
    RTorrent,

    /// SABnzbd usenet client
    Sabnzbd,

    /// NZBGet usenet client
    Nzbget,
}

impl DownloadClientType {
    /// Check if this is a torrent client
    pub fn is_torrent(&self) -> bool {
        matches!(
            self,
            DownloadClientType::QBittorrent
                | DownloadClientType::Transmission
                | DownloadClientType::Deluge
                | DownloadClientType::RTorrent
        )
    }

    /// Check if this is a usenet client
    pub fn is_usenet(&self) -> bool {
        matches!(
            self,
            DownloadClientType::Sabnzbd | DownloadClientType::Nzbget
        )
    }

    /// Get the default port for this client type
    pub fn default_port(&self) -> u16 {
        match self {
            DownloadClientType::QBittorrent => 8080,
            DownloadClientType::Transmission => 9091,
            DownloadClientType::Deluge => 8112,
            DownloadClientType::RTorrent => 80,
            DownloadClientType::Sabnzbd => 8080,
            DownloadClientType::Nzbget => 6789,
        }
    }

    /// Get a display name for this client type
    pub fn display_name(&self) -> &'static str {
        match self {
            DownloadClientType::QBittorrent => "qBittorrent",
            DownloadClientType::Transmission => "Transmission",
            DownloadClientType::Deluge => "Deluge",
            DownloadClientType::RTorrent => "rTorrent",
            DownloadClientType::Sabnzbd => "SABnzbd",
            DownloadClientType::Nzbget => "NZBGet",
        }
    }
}

/// Download client configuration
///
/// Represents a configured download client that can receive
/// and process downloads. Multiple clients can be configured,
/// with priority determining which is used first.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DownloadClient {
    /// Internal unique identifier
    pub id: Uuid,

    /// Display name for this client
    pub name: String,

    /// Client type
    pub client_type: DownloadClientType,

    /// Hostname or IP address
    pub host: String,

    /// Port number
    pub port: u16,

    /// Username for authentication (if required)
    pub username: Option<String>,

    /// Password for authentication (if required)
    pub password: Option<String>,

    /// API key (for clients that use it)
    pub api_key: Option<String>,

    /// Whether to use SSL/TLS
    pub use_ssl: bool,

    /// Base URL path (e.g., "/transmission/rpc")
    pub url_base: Option<String>,

    /// Whether this client is currently enabled
    pub enabled: bool,

    /// Priority for this client (higher = preferred)
    pub priority: i32,

    /// Download directory/category to use
    pub download_directory: Option<PathBuf>,

    /// Category or label to apply to downloads
    pub category: Option<String>,

    /// Timestamp when this client was added
    pub created_at: DateTime<Utc>,

    /// Timestamp of last configuration update
    pub updated_at: DateTime<Utc>,
}

impl DownloadClient {
    /// Create a new download client with default values
    pub fn new(
        name: String,
        client_type: DownloadClientType,
        host: String,
        port: u16,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            client_type,
            host,
            port,
            username: None,
            password: None,
            api_key: None,
            use_ssl: false,
            url_base: None,
            enabled: true,
            priority: 50,
            download_directory: None,
            category: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the base URL for this client
    pub fn base_url(&self) -> String {
        let protocol = if self.use_ssl { "https" } else { "http" };
        let mut url = format!("{}://{}:{}", protocol, self.host, self.port);

        if let Some(ref base) = self.url_base {
            url.push_str(base);
        }

        url
    }

    /// Check if this client is usable
    pub fn is_usable(&self) -> bool {
        self.enabled
    }
}

/// Download status
///
/// Tracks the current state of a download through its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "download_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    /// Download has been queued but not yet started
    Queued,

    /// Download is currently in progress
    Downloading,

    /// Download is paused
    Paused,

    /// Download completed successfully
    Completed,

    /// Download failed with an error
    Failed,

    /// Download is being imported/processed
    Importing,

    /// Download was imported successfully
    Imported,

    /// Download was removed/deleted
    Removed,
}

impl DownloadStatus {
    /// Check if this status represents an active download
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            DownloadStatus::Queued
                | DownloadStatus::Downloading
                | DownloadStatus::Paused
                | DownloadStatus::Importing
        )
    }

    /// Check if this status represents a completed state
    pub fn is_complete(&self) -> bool {
        matches!(
            self,
            DownloadStatus::Completed | DownloadStatus::Imported
        )
    }

    /// Check if this status represents a failure
    pub fn is_failed(&self) -> bool {
        matches!(self, DownloadStatus::Failed)
    }
}

/// Active download tracking
///
/// Represents a download that has been sent to a client.
/// This tracks the download through its lifecycle and links
/// it back to the media item it's for.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Download {
    /// Internal unique identifier
    pub id: Uuid,

    /// Download client that's handling this
    pub client_id: Uuid,

    /// Media item this download is for (optional - might be speculative)
    pub media_item_id: Option<Uuid>,

    /// Episode ID (if this is for a specific episode)
    pub episode_id: Option<Uuid>,

    /// Media type
    pub media_type: MediaType,

    /// Release title/name
    pub title: String,

    /// Download hash or ID from the client
    pub download_id: String,

    /// Current status
    pub status: DownloadStatus,

    /// Download progress (0-100)
    pub progress: f64,

    /// Download size in bytes
    pub size_bytes: Option<i64>,

    /// Downloaded bytes so far
    pub downloaded_bytes: Option<i64>,

    /// Download speed in bytes/second
    pub download_speed: Option<i64>,

    /// Estimated time remaining in seconds
    pub eta_seconds: Option<i64>,

    /// Error message (if failed)
    pub error_message: Option<String>,

    /// Output directory or file path
    pub output_path: Option<PathBuf>,

    /// Quality of this download
    pub quality: Option<String>,

    /// Timestamp when download was added
    pub added_at: DateTime<Utc>,

    /// Timestamp when download completed
    pub completed_at: Option<DateTime<Utc>>,

    /// Timestamp of last status update
    pub updated_at: DateTime<Utc>,
}

impl Download {
    /// Create a new download
    pub fn new(
        client_id: Uuid,
        media_type: MediaType,
        title: String,
        download_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            client_id,
            media_item_id: None,
            episode_id: None,
            media_type,
            title,
            download_id,
            status: DownloadStatus::Queued,
            progress: 0.0,
            size_bytes: None,
            downloaded_bytes: None,
            download_speed: None,
            eta_seconds: None,
            error_message: None,
            output_path: None,
            quality: None,
            added_at: now,
            completed_at: None,
            updated_at: now,
        }
    }

    /// Update download progress
    pub fn update_progress(&mut self, progress: f64, downloaded: i64, speed: i64, eta: i64) {
        self.progress = progress.clamp(0.0, 100.0);
        self.downloaded_bytes = Some(downloaded);
        self.download_speed = Some(speed);
        self.eta_seconds = Some(eta);
        self.updated_at = Utc::now();
    }

    /// Mark download as completed
    pub fn mark_completed(&mut self, output_path: PathBuf) {
        self.status = DownloadStatus::Completed;
        self.progress = 100.0;
        self.output_path = Some(output_path);
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Mark download as failed
    pub fn mark_failed(&mut self, error: String) {
        self.status = DownloadStatus::Failed;
        self.error_message = Some(error);
        self.updated_at = Utc::now();
    }

    /// Get progress as a percentage string
    pub fn progress_string(&self) -> String {
        format!("{:.1}%", self.progress)
    }

    /// Get human-readable size
    pub fn size_string(&self) -> Option<String> {
        self.size_bytes.map(|bytes| format_bytes(bytes))
    }

    /// Get human-readable download speed
    pub fn speed_string(&self) -> Option<String> {
        self.download_speed
            .map(|speed| format!("{}/s", format_bytes(speed)))
    }

    /// Get human-readable ETA
    pub fn eta_string(&self) -> Option<String> {
        self.eta_seconds.map(format_duration)
    }
}

/// Format bytes into human-readable size
fn format_bytes(bytes: i64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Format seconds into human-readable duration
fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_client_creation() {
        let client = DownloadClient::new(
            "My Client".to_string(),
            DownloadClientType::QBittorrent,
            "localhost".to_string(),
            8080,
        );

        assert_eq!(client.name, "My Client");
        assert_eq!(client.client_type, DownloadClientType::QBittorrent);
        assert!(client.enabled);
    }

    #[test]
    fn test_download_client_url() {
        let client = DownloadClient::new(
            "Test".to_string(),
            DownloadClientType::QBittorrent,
            "localhost".to_string(),
            8080,
        );

        assert_eq!(client.base_url(), "http://localhost:8080");
    }

    #[test]
    fn test_download_status() {
        assert!(DownloadStatus::Downloading.is_active());
        assert!(DownloadStatus::Completed.is_complete());
        assert!(DownloadStatus::Failed.is_failed());
        assert!(!DownloadStatus::Completed.is_active());
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1_048_576), "1.00 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3665), "1h 1m");
    }

    #[test]
    fn test_client_type_classification() {
        assert!(DownloadClientType::QBittorrent.is_torrent());
        assert!(DownloadClientType::Sabnzbd.is_usenet());
        assert!(!DownloadClientType::QBittorrent.is_usenet());
    }
}
