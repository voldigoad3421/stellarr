//! Indexer configuration and management
//!
//! Indexers are the sources for finding media releases. They can be:
//! - Usenet indexers (Newznab protocol)
//! - Torrent trackers (Torznab protocol)
//!
//! This module defines the configuration and metadata for indexers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Indexer protocol type
///
/// Determines how to communicate with the indexer and what
/// authentication/query methods to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "indexer_protocol", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum IndexerProtocol {
    /// Newznab protocol (usenet indexers)
    /// Uses standard Newznab API with capabilities check
    Newznab,

    /// Torznab protocol (torrent indexers)
    /// Extended Newznab protocol for torrents
    Torznab,
}

impl IndexerProtocol {
    /// Get the default API path for this protocol
    pub fn default_api_path(&self) -> &'static str {
        match self {
            IndexerProtocol::Newznab => "/api",
            IndexerProtocol::Torznab => "/api",
        }
    }

    /// Get the capabilities endpoint for this protocol
    pub fn capabilities_endpoint(&self) -> &'static str {
        match self {
            IndexerProtocol::Newznab => "?t=caps",
            IndexerProtocol::Torznab => "?t=caps",
        }
    }
}

/// Indexer configuration
///
/// Represents a configured indexer that can be queried for releases.
/// Each indexer has its own API key, priority, and category mappings.
///
/// Design decisions:
/// - Priority determines query order (higher = queried first)
/// - Categories are indexer-specific numeric IDs
/// - Enabled flag allows temporary disable without deletion
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Indexer {
    /// Internal unique identifier
    pub id: Uuid,

    /// Display name for this indexer
    pub name: String,

    /// Protocol type (Newznab or Torznab)
    pub protocol: IndexerProtocol,

    /// Base URL of the indexer (without /api path)
    pub url: String,

    /// API key for authentication
    /// Stored as an Option to allow indexers without keys (rare)
    pub api_key: Option<String>,

    /// Whether this indexer is currently enabled
    pub enabled: bool,

    /// Query priority (1-100, higher = higher priority)
    /// Higher priority indexers are queried first
    pub priority: i32,

    /// Category IDs to search
    /// These are indexer-specific numeric IDs
    /// Common: 2000=Movies, 5000=TV, 8000=Audio
    pub categories: Vec<i32>,

    /// Additional capabilities (parsed from caps endpoint)
    /// Stored as JSON for flexibility
    pub capabilities: Option<serde_json::Value>,

    /// Timestamp of last successful query
    pub last_used_at: Option<DateTime<Utc>>,

    /// Timestamp when this indexer was added
    pub created_at: DateTime<Utc>,

    /// Timestamp of last configuration update
    pub updated_at: DateTime<Utc>,
}

impl Indexer {
    /// Create a new indexer with default values
    pub fn new(name: String, protocol: IndexerProtocol, url: String, api_key: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            protocol,
            url,
            api_key: Some(api_key),
            enabled: true,
            priority: 50, // Default middle priority
            categories: match protocol {
                IndexerProtocol::Newznab => vec![2000, 5000], // Movies + TV
                IndexerProtocol::Torznab => vec![2000, 5000],
            },
            capabilities: None,
            last_used_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the full API URL for this indexer
    pub fn api_url(&self) -> String {
        let base = self.url.trim_end_matches('/');
        format!("{}{}", base, self.protocol.default_api_path())
    }

    /// Get the capabilities check URL
    pub fn capabilities_url(&self) -> String {
        format!("{}{}", self.api_url(), self.protocol.capabilities_endpoint())
    }

    /// Build a search URL with parameters
    pub fn search_url(&self, params: &IndexerSearchParams) -> String {
        let mut url = format!("{}?t=search", self.api_url());

        if let Some(ref api_key) = self.api_key {
            url.push_str(&format!("&apikey={}", api_key));
        }

        if let Some(ref query) = params.query {
            url.push_str(&format!("&q={}", urlencoding::encode(query)));
        }

        if let Some(ref imdb_id) = params.imdb_id {
            url.push_str(&format!("&imdbid={}", imdb_id));
        }

        if let Some(ref tvdb_id) = params.tvdb_id {
            url.push_str(&format!("&tvdbid={}", tvdb_id));
        }

        if let Some(season) = params.season {
            url.push_str(&format!("&season={}", season));
        }

        if let Some(episode) = params.episode {
            url.push_str(&format!("&ep={}", episode));
        }

        // Add categories
        for cat in &self.categories {
            url.push_str(&format!("&cat={}", cat));
        }

        url
    }

    /// Mark this indexer as recently used
    pub fn mark_used(&mut self) {
        self.last_used_at = Some(Utc::now());
    }

    /// Check if this indexer is usable
    pub fn is_usable(&self) -> bool {
        self.enabled && self.api_key.is_some()
    }
}

/// Search parameters for indexer queries
///
/// Different indexers support different search methods.
/// This struct provides all possible search parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexerSearchParams {
    /// Text query (title search)
    pub query: Option<String>,

    /// IMDb ID (e.g., "tt1234567")
    pub imdb_id: Option<String>,

    /// TVDb ID for TV shows
    pub tvdb_id: Option<i64>,

    /// TMDb ID
    pub tmdb_id: Option<i64>,

    /// Season number (for TV)
    pub season: Option<i32>,

    /// Episode number (for TV)
    pub episode: Option<i32>,

    /// Limit number of results
    pub limit: Option<i32>,

    /// Offset for pagination
    pub offset: Option<i32>,
}

impl IndexerSearchParams {
    /// Create a new search params with a text query
    pub fn query(query: impl Into<String>) -> Self {
        Self {
            query: Some(query.into()),
            ..Default::default()
        }
    }

    /// Create search params for a movie by IMDb ID
    pub fn movie_by_imdb(imdb_id: impl Into<String>) -> Self {
        Self {
            imdb_id: Some(imdb_id.into()),
            ..Default::default()
        }
    }

    /// Create search params for a TV episode
    pub fn tv_episode(tvdb_id: i64, season: i32, episode: i32) -> Self {
        Self {
            tvdb_id: Some(tvdb_id),
            season: Some(season),
            episode: Some(episode),
            ..Default::default()
        }
    }

    /// Create search params for a TV season
    pub fn tv_season(tvdb_id: i64, season: i32) -> Self {
        Self {
            tvdb_id: Some(tvdb_id),
            season: Some(season),
            ..Default::default()
        }
    }

    /// Set the result limit
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Indexer statistics
///
/// Tracks usage and performance metrics for an indexer.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IndexerStats {
    /// Indexer ID
    pub indexer_id: Uuid,

    /// Total number of queries made
    pub total_queries: i64,

    /// Number of successful queries
    pub successful_queries: i64,

    /// Number of failed queries
    pub failed_queries: i64,

    /// Total number of results returned
    pub total_results: i64,

    /// Number of results that were grabbed/downloaded
    pub grabbed_results: i64,

    /// Average response time in milliseconds
    pub avg_response_time_ms: Option<i32>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl IndexerStats {
    /// Create new stats for an indexer
    pub fn new(indexer_id: Uuid) -> Self {
        Self {
            indexer_id,
            total_queries: 0,
            successful_queries: 0,
            failed_queries: 0,
            total_results: 0,
            grabbed_results: 0,
            avg_response_time_ms: None,
            updated_at: Utc::now(),
        }
    }

    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_queries == 0 {
            return 0.0;
        }
        (self.successful_queries as f64 / self.total_queries as f64) * 100.0
    }

    /// Calculate grab rate (how many results were actually used)
    pub fn grab_rate(&self) -> f64 {
        if self.total_results == 0 {
            return 0.0;
        }
        (self.grabbed_results as f64 / self.total_results as f64) * 100.0
    }
}

/// Dependency for URL encoding in search URLs
///
/// Note: This is a minimal inline implementation to avoid adding a dependency.
/// In production, you might want to use the `urlencoding` crate.
mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexer_creation() {
        let indexer = Indexer::new(
            "Test Indexer".to_string(),
            IndexerProtocol::Newznab,
            "https://indexer.example.com".to_string(),
            "test-api-key".to_string(),
        );

        assert_eq!(indexer.name, "Test Indexer");
        assert_eq!(indexer.protocol, IndexerProtocol::Newznab);
        assert!(indexer.enabled);
        assert_eq!(indexer.priority, 50);
    }

    #[test]
    fn test_indexer_api_url() {
        let indexer = Indexer::new(
            "Test".to_string(),
            IndexerProtocol::Newznab,
            "https://indexer.example.com".to_string(),
            "key".to_string(),
        );

        assert_eq!(indexer.api_url(), "https://indexer.example.com/api");
    }

    #[test]
    fn test_indexer_search_params() {
        let params = IndexerSearchParams::tv_episode(12345, 1, 5);
        assert_eq!(params.tvdb_id, Some(12345));
        assert_eq!(params.season, Some(1));
        assert_eq!(params.episode, Some(5));
    }

    #[test]
    fn test_indexer_stats() {
        let stats = IndexerStats {
            indexer_id: Uuid::new_v4(),
            total_queries: 100,
            successful_queries: 95,
            failed_queries: 5,
            total_results: 500,
            grabbed_results: 50,
            avg_response_time_ms: Some(250),
            updated_at: Utc::now(),
        };

        assert_eq!(stats.success_rate(), 95.0);
        assert_eq!(stats.grab_rate(), 10.0);
    }
}
