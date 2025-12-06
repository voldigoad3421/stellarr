use async_trait::async_trait;
use reqwest::Client;

use super::{Indexer, SearchResult};
use stellarr_core::Result;

/// Prowlarr indexer client
pub struct ProwlarrClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ProwlarrClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }
}

#[async_trait]
impl Indexer for ProwlarrClient {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        // TODO: Implement Prowlarr API search
        todo!("Implement Prowlarr search")
    }

    async fn test_connection(&self) -> Result<bool> {
        // TODO: Implement connection test
        todo!("Implement connection test")
    }
}
