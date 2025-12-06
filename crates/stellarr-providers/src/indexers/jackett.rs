use async_trait::async_trait;
use reqwest::Client;

use super::{Indexer, SearchResult};
use stellarr_core::Result;

/// Jackett indexer client
pub struct JackettClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl JackettClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }
}

#[async_trait]
impl Indexer for JackettClient {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        // TODO: Implement Jackett Torznab search
        todo!("Implement Jackett search")
    }

    async fn test_connection(&self) -> Result<bool> {
        // TODO: Implement connection test
        todo!("Implement connection test")
    }
}
