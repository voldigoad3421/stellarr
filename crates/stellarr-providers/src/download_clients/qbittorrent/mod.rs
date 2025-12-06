//! qBittorrent Web API client implementation
//!
//! This module provides a complete implementation of the qBittorrent Web API v2,
//! allowing you to manage torrents programmatically.
//!
//! # Example
//!
//! ```no_run
//! use stellarr_providers::download_clients::qbittorrent::QBittorrentClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = QBittorrentClient::new(
//!         "localhost".to_string(),
//!         8080,
//!         "admin".to_string(),
//!         "adminpass".to_string(),
//!     )?;
//!
//!     // Login
//!     client.login().await?;
//!
//!     // Add a torrent
//!     let magnet = "magnet:?xt=urn:btih:...";
//!     client.add_torrent(magnet, Some("movies"), false).await?;
//!
//!     // List torrents
//!     let torrents = client.get_torrents(None).await?;
//!     for torrent in torrents {
//!         println!("{}: {}%", torrent.name, torrent.progress * 100.0);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

pub use client::QBittorrentClient;
pub use error::{QBittorrentError, Result};
pub use models::{Torrent, TorrentProperties, TorrentState};
