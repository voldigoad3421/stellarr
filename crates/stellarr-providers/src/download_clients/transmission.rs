use async_trait::async_trait;
use reqwest::Client;

use super::{DownloadClient, DownloadStatus};
use stellarr_core::Result;

/// Transmission client
pub struct TransmissionClient {
    client: Client,
    base_url: String,
    username: Option<String>,
    password: Option<String>,
}

impl TransmissionClient {
    pub fn new(base_url: String, username: Option<String>, password: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url,
            username,
            password,
        }
    }
}

#[async_trait]
impl DownloadClient for TransmissionClient {
    async fn add_torrent(&self, url: &str, save_path: &str) -> Result<String> {
        // TODO: Implement Transmission RPC call
        todo!("Implement add torrent")
    }

    async fn get_status(&self, id: &str) -> Result<DownloadStatus> {
        todo!("Implement get status")
    }

    async fn remove(&self, id: &str, delete_files: bool) -> Result<()> {
        todo!("Implement remove torrent")
    }

    async fn test_connection(&self) -> Result<bool> {
        todo!("Implement connection test")
    }
}
