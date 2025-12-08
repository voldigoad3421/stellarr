use async_trait::async_trait;
use reqwest::{Client, multipart};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::error::{QBittorrentError, Result};
use super::models::{Torrent, TorrentProperties};
use crate::download_clients::{DownloadClient, DownloadState, DownloadStatus};

/// qBittorrent Web API client
#[derive(Clone)]
pub struct QBittorrentClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
    authenticated: Arc<RwLock<bool>>,
}

impl QBittorrentClient {
    /// Create a new qBittorrent client
    ///
    /// # Arguments
    ///
    /// * `host` - qBittorrent host (e.g., "localhost", "192.168.1.100")
    /// * `port` - qBittorrent Web UI port (default: 8080)
    /// * `username` - Web UI username
    /// * `password` - Web UI password
    pub fn new(host: String, port: u16, username: String, password: String) -> Result<Self> {
        // Build base URL
        let base_url = if host.starts_with("http://") || host.starts_with("https://") {
            format!("{}/api/v2", host.trim_end_matches('/'))
        } else {
            format!("http://{}:{}/api/v2", host, port)
        };

        // Create HTTP client with cookie store enabled
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|e| QBittorrentError::RequestFailed(e))?;

        Ok(Self {
            client,
            base_url,
            username,
            password,
            authenticated: Arc::new(RwLock::new(false)),
        })
    }

    /// Authenticate with qBittorrent and store session cookie
    pub async fn login(&self) -> Result<()> {
        let url = format!("{}/auth/login", self.base_url);

        debug!("Logging in to qBittorrent at {}", url);

        let params = [
            ("username", self.username.as_str()),
            ("password", self.password.as_str()),
        ];

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() && text == "Ok." {
            info!("Successfully authenticated with qBittorrent");
            *self.authenticated.write().await = true;
            Ok(())
        } else {
            error!("Authentication failed: {}", text);
            Err(QBittorrentError::AuthenticationFailed(text))
        }
    }

    /// Ensure we're authenticated, login if necessary
    async fn ensure_authenticated(&self) -> Result<()> {
        if !*self.authenticated.read().await {
            self.login().await?;
        }
        Ok(())
    }

    /// Add a torrent by URL or magnet link
    ///
    /// # Arguments
    ///
    /// * `url` - Torrent URL or magnet link
    /// * `category` - Optional category to assign
    /// * `paused` - Whether to add the torrent in paused state
    pub async fn add_torrent(
        &self,
        url: &str,
        category: Option<&str>,
        paused: bool,
    ) -> Result<String> {
        self.ensure_authenticated().await?;

        let endpoint = format!("{}/torrents/add", self.base_url);

        debug!("Adding torrent from URL: {}", url);

        let mut form = multipart::Form::new()
            .text("urls", url.to_string())
            .text("paused", paused.to_string());

        if let Some(cat) = category {
            form = form.text("category", cat.to_string());
        }

        let response = self
            .client
            .post(&endpoint)
            .multipart(form)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() && text == "Ok." {
            info!("Successfully added torrent from URL");
            // qBittorrent doesn't return hash directly, we'll need to find it
            // For now, return the URL as identifier
            Ok(url.to_string())
        } else {
            error!("Failed to add torrent: {}", text);
            Err(QBittorrentError::AddTorrentFailed(text))
        }
    }

    /// Add a torrent from file data
    ///
    /// # Arguments
    ///
    /// * `data` - Torrent file contents
    /// * `filename` - Original filename
    /// * `category` - Optional category to assign
    pub async fn add_torrent_file(
        &self,
        data: Vec<u8>,
        filename: &str,
        category: Option<&str>,
    ) -> Result<String> {
        self.ensure_authenticated().await?;

        let endpoint = format!("{}/torrents/add", self.base_url);

        debug!("Adding torrent from file: {}", filename);

        let file_part = multipart::Part::bytes(data)
            .file_name(filename.to_string())
            .mime_str("application/x-bittorrent")
            .map_err(|e| QBittorrentError::InvalidResponse(e.to_string()))?;

        let mut form = multipart::Form::new().part("torrents", file_part);

        if let Some(cat) = category {
            form = form.text("category", cat.to_string());
        }

        let response = self
            .client
            .post(&endpoint)
            .multipart(form)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() && text == "Ok." {
            info!("Successfully added torrent from file");
            Ok(filename.to_string())
        } else {
            error!("Failed to add torrent from file: {}", text);
            Err(QBittorrentError::AddTorrentFailed(text))
        }
    }

    /// Get list of torrents, optionally filtered by category
    ///
    /// # Arguments
    ///
    /// * `category` - Optional category filter
    pub async fn get_torrents(&self, category: Option<&str>) -> Result<Vec<Torrent>> {
        self.ensure_authenticated().await?;

        let url = format!("{}/torrents/info", self.base_url);

        debug!("Fetching torrents from {}", url);

        let mut request = self.client.get(&url);

        if let Some(cat) = category {
            request = request.query(&[("category", cat)]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to get torrents: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let torrents: Vec<Torrent> = response.json().await?;
        debug!("Retrieved {} torrents", torrents.len());

        Ok(torrents)
    }

    /// Get single torrent info by hash
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash (infohash)
    pub async fn get_torrent(&self, hash: &str) -> Result<Torrent> {
        self.ensure_authenticated().await?;

        let url = format!("{}/torrents/info?hashes={}", self.base_url, hash);

        debug!("Fetching torrent info for hash: {}", hash);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to get torrent: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let mut torrents: Vec<Torrent> = response.json().await?;

        if torrents.is_empty() {
            warn!("Torrent not found: {}", hash);
            return Err(QBittorrentError::TorrentNotFound(hash.to_string()));
        }

        Ok(torrents.remove(0))
    }

    /// Get detailed torrent properties
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash (infohash)
    pub async fn get_torrent_properties(&self, hash: &str) -> Result<TorrentProperties> {
        self.ensure_authenticated().await?;

        let url = format!("{}/torrents/properties?hash={}", self.base_url, hash);

        debug!("Fetching torrent properties for hash: {}", hash);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to get torrent properties: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let properties: TorrentProperties = response.json().await?;

        Ok(properties)
    }

    /// Delete torrent(s)
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash (infohash), can be multiple separated by |
    /// * `delete_files` - Whether to delete downloaded files
    pub async fn delete_torrent(&self, hash: &str, delete_files: bool) -> Result<()> {
        self.ensure_authenticated().await?;

        let url = format!("{}/torrents/delete", self.base_url);

        debug!("Deleting torrent: {} (delete_files: {})", hash, delete_files);

        let params = [
            ("hashes", hash),
            ("deleteFiles", if delete_files { "true" } else { "false" }),
        ];

        let response = self.client.post(&url).form(&params).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to delete torrent: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        info!("Successfully deleted torrent: {}", hash);
        Ok(())
    }

    /// Pause torrent
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash (infohash)
    pub async fn pause_torrent(&self, hash: &str) -> Result<()> {
        self.ensure_authenticated().await?;

        let url = format!("{}/torrents/pause", self.base_url);

        debug!("Pausing torrent: {}", hash);

        let params = [("hashes", hash)];

        let response = self.client.post(&url).form(&params).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to pause torrent: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        info!("Successfully paused torrent: {}", hash);
        Ok(())
    }

    /// Resume torrent
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash (infohash)
    pub async fn resume_torrent(&self, hash: &str) -> Result<()> {
        self.ensure_authenticated().await?;

        let url = format!("{}/torrents/resume", self.base_url);

        debug!("Resuming torrent: {}", hash);

        let params = [("hashes", hash)];

        let response = self.client.post(&url).form(&params).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to resume torrent: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        info!("Successfully resumed torrent: {}", hash);
        Ok(())
    }

    /// Get qBittorrent application version
    pub async fn get_version(&self) -> Result<String> {
        self.ensure_authenticated().await?;

        let url = format!("{}/app/version", self.base_url);

        debug!("Fetching qBittorrent version");

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to get version: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let version = response.text().await?;
        debug!("qBittorrent version: {}", version);

        Ok(version)
    }

    /// Get Web API version
    pub async fn get_api_version(&self) -> Result<String> {
        self.ensure_authenticated().await?;

        let url = format!("{}/app/webapiVersion", self.base_url);

        debug!("Fetching Web API version");

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to get API version: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        let version = response.text().await?;
        debug!("Web API version: {}", version);

        Ok(version)
    }

    /// Set torrent category
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash (infohash)
    /// * `category` - Category name
    pub async fn set_category(&self, hash: &str, category: &str) -> Result<()> {
        self.ensure_authenticated().await?;

        let url = format!("{}/torrents/setCategory", self.base_url);

        debug!("Setting category '{}' for torrent: {}", category, hash);

        let params = [("hashes", hash), ("category", category)];

        let response = self.client.post(&url).form(&params).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            error!("Failed to set category: {} - {}", status, text);
            return Err(QBittorrentError::ApiError(format!(
                "Status {}: {}",
                status, text
            )));
        }

        info!("Successfully set category for torrent: {}", hash);
        Ok(())
    }

    /// Convert qBittorrent TorrentState to DownloadState
    fn convert_state(state: &super::models::TorrentState) -> DownloadState {
        if state.is_error() {
            DownloadState::Error
        } else if state.is_paused() {
            DownloadState::Paused
        } else if state.is_downloading() {
            DownloadState::Downloading
        } else if state.is_seeding() {
            DownloadState::Seeding
        } else if state.is_completed() {
            DownloadState::Completed
        } else {
            DownloadState::Queued
        }
    }
}

#[async_trait]
impl DownloadClient for QBittorrentClient {
    async fn add_torrent(&self, url: &str, save_path: &str) -> stellarr_core::Result<String> {
        // Add torrent and get hash
        let result = self
            .add_torrent(url, None, false)
            .await
            .map_err(|e| stellarr_core::Error::ExternalService(e.to_string()))?;

        // Wait a moment for torrent to be added
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Try to find the torrent by matching the URL/magnet
        let torrents = self
            .get_torrents(None)
            .await
            .map_err(|e| stellarr_core::Error::ExternalService(e.to_string()))?;

        // If it's a magnet link, extract the hash from the URL
        if url.starts_with("magnet:?") {
            if let Some(hash_start) = url.find("urn:btih:") {
                let hash_part = &url[hash_start + 9..];
                let hash = hash_part
                    .split('&')
                    .next()
                    .unwrap_or(hash_part)
                    .to_lowercase();

                // Try to find torrent with matching hash
                for torrent in &torrents {
                    if torrent.hash.to_lowercase().starts_with(&hash) {
                        return Ok(torrent.hash.clone());
                    }
                }
            }
        }

        // If we can't find it, return the most recently added torrent
        if let Some(torrent) = torrents.first() {
            Ok(torrent.hash.clone())
        } else {
            Ok(result)
        }
    }

    async fn get_status(&self, id: &str) -> stellarr_core::Result<DownloadStatus> {
        let torrent = self
            .get_torrent(id)
            .await
            .map_err(|e| stellarr_core::Error::ExternalService(e.to_string()))?;

        let eta = if torrent.eta > 0 {
            Some(torrent.eta as u64)
        } else {
            None
        };

        Ok(DownloadStatus {
            id: torrent.hash,
            name: torrent.name,
            progress: torrent.progress,
            download_speed: torrent.dlspeed,
            eta,
            state: Self::convert_state(&torrent.state),
        })
    }

    async fn remove(&self, id: &str, delete_files: bool) -> stellarr_core::Result<()> {
        self.delete_torrent(id, delete_files)
            .await
            .map_err(|e| stellarr_core::Error::ExternalService(e.to_string()))
    }

    async fn test_connection(&self) -> stellarr_core::Result<bool> {
        match self.login().await {
            Ok(_) => {
                // Try to get version as additional verification
                match self.get_version().await {
                    Ok(_) => Ok(true),
                    Err(e) => {
                        error!("Version check failed: {}", e);
                        Ok(false)
                    }
                }
            }
            Err(e) => {
                error!("Connection test failed: {}", e);
                Ok(false)
            }
        }
    }
}
