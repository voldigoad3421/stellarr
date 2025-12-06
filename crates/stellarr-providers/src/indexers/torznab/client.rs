//! Torznab/Newznab API client implementation

use super::error::TorznabError;
use super::models::{Capabilities, Item, TorznabResponse};
use reqwest::Client;
use std::collections::HashMap;
use tracing::{debug, trace, warn};

/// Torznab/Newznab API client
///
/// This client implements the Torznab/Newznab protocol used by indexers
/// like Jackett, Prowlarr, and native Newznab indexers.
#[derive(Debug, Clone)]
pub struct TorznabClient {
    client: Client,
    base_url: String,
    api_key: String,
    indexer_name: String,
}

impl TorznabClient {
    /// Create a new Torznab client
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL of the indexer (e.g., "http://localhost:9117/api/v2.0/indexers/all")
    /// * `api_key` - API key for authentication
    /// * `indexer_name` - Human-readable name for this indexer (for logging)
    pub fn new(base_url: String, api_key: String, indexer_name: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            indexer_name,
        }
    }

    /// Fetch indexer capabilities
    ///
    /// Queries the `?t=caps` endpoint to discover what search types and
    /// categories the indexer supports.
    pub async fn capabilities(&self) -> Result<Capabilities, TorznabError> {
        debug!(
            indexer = %self.indexer_name,
            "Fetching capabilities"
        );

        let url = format!("{}/api?t=caps&apikey={}", self.base_url, self.api_key);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                warn!(
                    indexer = %self.indexer_name,
                    error = %e,
                    "Failed to fetch capabilities"
                );
                TorznabError::HttpError(e)
            })?;

        let status = response.status();
        let text = response.text().await.map_err(TorznabError::HttpError)?;

        if !status.is_success() {
            return Err(TorznabError::IndexerError {
                code: status.as_str().to_string(),
                description: text,
            });
        }

        trace!(xml = %text, "Capabilities XML response");

        quick_xml::de::from_str(&text).map_err(|e| {
            warn!(
                indexer = %self.indexer_name,
                error = %e,
                xml = %text,
                "Failed to parse capabilities XML"
            );
            TorznabError::XmlParseError(e)
        })
    }

    /// Perform a basic search
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string
    /// * `categories` - Optional list of category IDs to filter results
    /// * `limit` - Maximum number of results (defaults to 100)
    /// * `offset` - Result offset for pagination (defaults to 0)
    pub async fn search(
        &self,
        query: &str,
        categories: Option<&[u32]>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Item>, TorznabError> {
        let mut params = HashMap::new();
        params.insert("t", "search".to_string());
        params.insert("apikey", self.api_key.clone());
        params.insert("q", query.to_string());

        if let Some(cats) = categories {
            let cat_str = cats
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.insert("cat", cat_str);
        }

        if let Some(lim) = limit {
            params.insert("limit", lim.to_string());
        }

        if let Some(off) = offset {
            params.insert("offset", off.to_string());
        }

        self.execute_search(params).await
    }

    /// Perform a TV show search
    ///
    /// # Arguments
    ///
    /// * `tvdb_id` - TVDB ID of the show
    /// * `season` - Season number (optional)
    /// * `episode` - Episode number (optional, requires season)
    /// * `query` - Additional search query (optional)
    /// * `categories` - Optional list of category IDs
    pub async fn tv_search(
        &self,
        tvdb_id: Option<u32>,
        season: Option<u32>,
        episode: Option<u32>,
        query: Option<&str>,
        categories: Option<&[u32]>,
    ) -> Result<Vec<Item>, TorznabError> {
        let mut params = HashMap::new();
        params.insert("t", "tvsearch".to_string());
        params.insert("apikey", self.api_key.clone());

        if let Some(id) = tvdb_id {
            params.insert("tvdbid", id.to_string());
        }

        if let Some(s) = season {
            params.insert("season", s.to_string());
        }

        if let Some(ep) = episode {
            if season.is_none() {
                return Err(TorznabError::MissingParameter(
                    "season is required when episode is specified".to_string(),
                ));
            }
            params.insert("ep", ep.to_string());
        }

        if let Some(q) = query {
            params.insert("q", q.to_string());
        }

        if let Some(cats) = categories {
            let cat_str = cats
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.insert("cat", cat_str);
        }

        self.execute_search(params).await
    }

    /// Perform a movie search
    ///
    /// # Arguments
    ///
    /// * `imdb_id` - IMDB ID (without "tt" prefix)
    /// * `tmdb_id` - TMDB ID
    /// * `query` - Additional search query (optional)
    /// * `categories` - Optional list of category IDs
    pub async fn movie_search(
        &self,
        imdb_id: Option<&str>,
        tmdb_id: Option<u32>,
        query: Option<&str>,
        categories: Option<&[u32]>,
    ) -> Result<Vec<Item>, TorznabError> {
        let mut params = HashMap::new();
        params.insert("t", "movie".to_string());
        params.insert("apikey", self.api_key.clone());

        if let Some(id) = imdb_id {
            // Remove "tt" prefix if present
            let clean_id = id.trim_start_matches("tt");
            params.insert("imdbid", clean_id.to_string());
        }

        if let Some(id) = tmdb_id {
            params.insert("tmdbid", id.to_string());
        }

        if let Some(q) = query {
            params.insert("q", q.to_string());
        }

        if let Some(cats) = categories {
            let cat_str = cats
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.insert("cat", cat_str);
        }

        self.execute_search(params).await
    }

    /// Execute a search with the given parameters
    async fn execute_search(
        &self,
        params: HashMap<&str, String>,
    ) -> Result<Vec<Item>, TorznabError> {
        debug!(
            indexer = %self.indexer_name,
            search_type = ?params.get("t"),
            "Executing search"
        );

        let url = format!("{}/api", self.base_url);

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| {
                warn!(
                    indexer = %self.indexer_name,
                    error = %e,
                    "Search request failed"
                );
                TorznabError::HttpError(e)
            })?;

        let status = response.status();
        let text = response.text().await.map_err(TorznabError::HttpError)?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(error_info) = self.parse_error(&text) {
                return Err(error_info);
            }

            return Err(TorznabError::IndexerError {
                code: status.as_str().to_string(),
                description: text,
            });
        }

        trace!(xml = %text, "Search XML response");

        let response: TorznabResponse = quick_xml::de::from_str(&text).map_err(|e| {
            warn!(
                indexer = %self.indexer_name,
                error = %e,
                xml = %text,
                "Failed to parse search XML"
            );
            TorznabError::XmlParseError(e)
        })?;

        debug!(
            indexer = %self.indexer_name,
            count = response.channel.items.len(),
            "Search completed"
        );

        Ok(response.channel.items)
    }

    /// Parse error response from indexer
    fn parse_error(&self, xml: &str) -> Result<TorznabError, TorznabError> {
        // Try to parse as error XML
        #[derive(Deserialize)]
        struct ErrorResponse {
            #[serde(rename = "@code")]
            code: String,
            #[serde(rename = "@description")]
            description: String,
        }

        if let Ok(error) = quick_xml::de::from_str::<ErrorResponse>(xml) {
            return Ok(TorznabError::IndexerError {
                code: error.code,
                description: error.description,
            });
        }

        Err(TorznabError::InvalidResponse(xml.to_string()))
    }

    /// Test the connection to the indexer
    pub async fn test(&self) -> Result<bool, TorznabError> {
        debug!(indexer = %self.indexer_name, "Testing connection");

        // Try to fetch capabilities as a connection test
        match self.capabilities().await {
            Ok(_) => {
                debug!(indexer = %self.indexer_name, "Connection test passed");
                Ok(true)
            }
            Err(e) => {
                warn!(
                    indexer = %self.indexer_name,
                    error = %e,
                    "Connection test failed"
                );
                Err(e)
            }
        }
    }

    /// Get the indexer name
    pub fn name(&self) -> &str {
        &self.indexer_name
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TorznabClient::new(
            "http://localhost:9117/api/v2.0/indexers/all".to_string(),
            "test-api-key".to_string(),
            "TestIndexer".to_string(),
        );

        assert_eq!(client.name(), "TestIndexer");
        assert_eq!(
            client.base_url(),
            "http://localhost:9117/api/v2.0/indexers/all"
        );
    }

    #[tokio::test]
    async fn test_tv_search_validation() {
        let client = TorznabClient::new(
            "http://localhost:9117".to_string(),
            "test-key".to_string(),
            "Test".to_string(),
        );

        // Episode without season should fail
        let result = client
            .tv_search(Some(12345), None, Some(1), None, None)
            .await;

        assert!(result.is_err());
        match result {
            Err(TorznabError::MissingParameter(_)) => {}
            _ => panic!("Expected MissingParameter error"),
        }
    }
}
