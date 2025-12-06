// Indexer integrations (Jackett, Prowlarr) for searching torrents/usenet
// Will support Torznab/Newznab APIs

pub mod jackett;
pub mod prowlarr;
pub mod torznab;
pub mod quality_parser;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use stellarr_core::Result;

/// Search result from an indexer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub download_url: String,
    pub size: u64,
    pub seeders: Option<u32>,
    pub leechers: Option<u32>,
    pub indexer: String,
    pub quality: String,
}

/// Trait for indexer implementations
#[async_trait]
pub trait Indexer: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>>;
    async fn test_connection(&self) -> Result<bool>;
}
