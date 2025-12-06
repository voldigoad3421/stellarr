use async_trait::async_trait;
use reqwest::Client;

use super::{DownloadClient, DownloadStatus};
use stellarr_core::Result;

/// SABnzbd client (Usenet downloader)
pub struct SabnzbdClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl SabnzbdClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }
}

#[async_trait]
impl DownloadClient for SabnzbdClient {
    async fn add_torrent(&self, url: &str, save_path: &str) -> Result<String> {
        // Note: SABnzbd handles NZB files, not torrents
        // TODO: Implement NZB addition
        todo!("Implement add NZB")
    }

    async fn get_status(&self, id: &str) -> Result<DownloadStatus> {
        todo!("Implement get status")
    }

    async fn remove(&self, id: &str, delete_files: bool) -> Result<()> {
        todo!("Implement remove download")
    }

    async fn test_connection(&self) -> Result<bool> {
        todo!("Implement connection test")
    }
}
