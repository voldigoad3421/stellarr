// Download client integrations (qBittorrent, Transmission, SABnzbd)

pub mod qbittorrent;
pub mod transmission;
pub mod sabnzbd;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use stellarr_core::Result;

/// Download client status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStatus {
    pub id: String,
    pub name: String,
    pub progress: f32,
    pub download_speed: u64,
    pub eta: Option<u64>,
    pub state: DownloadState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadState {
    Queued,
    Downloading,
    Seeding,
    Paused,
    Completed,
    Error,
}

/// Trait for download client implementations
#[async_trait]
pub trait DownloadClient: Send + Sync {
    async fn add_torrent(&self, url: &str, save_path: &str) -> Result<String>;
    async fn get_status(&self, id: &str) -> Result<DownloadStatus>;
    async fn remove(&self, id: &str, delete_files: bool) -> Result<()>;
    async fn test_connection(&self) -> Result<bool>;
}
