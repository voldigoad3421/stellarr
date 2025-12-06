use reqwest::{Client, StatusCode};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use super::error::{Result, TmdbError};
use super::models::{
    Configuration, Episode, Movie, MovieSearchResponse, Season, TvDetails, TvSearchResponse,
    TvShow,
};

const TMDB_API_BASE: &str = "https://api.themoviedb.org/3";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// TMDB API client for fetching movie and TV show metadata
pub struct TmdbClient {
    client: Client,
    access_token: String,
    config: Arc<RwLock<Option<Configuration>>>,
}

impl TmdbClient {
    /// Create a new TMDB client with an access token (Bearer token)
    pub fn new(access_token: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token))
                        .map_err(|e| TmdbError::Authentication)?,
                );
                headers.insert(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                headers
            })
            .build()?;

        Ok(Self {
            client,
            access_token,
            config: Arc::new(RwLock::new(None)),
        })
    }

    /// Load TMDB configuration (image base URLs and sizes)
    pub async fn load_config(&self) -> Result<Configuration> {
        // Check if we already have config cached
        {
            let config_read = self.config.read().await;
            if let Some(config) = config_read.as_ref() {
                return Ok(config.clone());
            }
        }

        // Fetch config from API
        let config: Configuration = self.get("/configuration").await?;

        // Cache it
        {
            let mut config_write = self.config.write().await;
            *config_write = Some(config.clone());
        }

        Ok(config)
    }

    /// Search for movies by query
    pub async fn search_movies(&self, query: &str, page: Option<u32>) -> Result<MovieSearchResponse> {
        let page = page.unwrap_or(1);
        self.get(&format!("/search/movie?query={}&page={}",
            urlencoding::encode(query),
            page
        ))
        .await
    }

    /// Search for TV shows by query
    pub async fn search_tv(&self, query: &str, page: Option<u32>) -> Result<TvSearchResponse> {
        let page = page.unwrap_or(1);
        self.get(&format!("/search/tv?query={}&page={}",
            urlencoding::encode(query),
            page
        ))
        .await
    }

    /// Get detailed movie information by TMDB ID
    pub async fn get_movie_details(&self, id: i64) -> Result<Movie> {
        self.get(&format!("/movie/{}?append_to_response=external_ids", id))
            .await
    }

    /// Get detailed TV show information by TMDB ID
    /// If seasons is true, includes season and episode details
    pub async fn get_tv_details(&self, id: i64, seasons: bool) -> Result<TvDetails> {
        let mut url = format!("/tv/{}?append_to_response=external_ids", id);

        if seasons {
            // Fetch the basic details first to know how many seasons there are
            let basic_details: TvDetails = self.get(&url).await?;

            // Now fetch each season's details
            let mut tv_details = basic_details;
            let mut seasons_with_episodes = Vec::new();

            for season in &tv_details.seasons {
                if season.season_number >= 0 {
                    // Fetch season details with episodes
                    match self.get_season_details(id, season.season_number).await {
                        Ok(season_detail) => seasons_with_episodes.push(season_detail),
                        Err(e) => {
                            // Log error but continue with other seasons
                            tracing::warn!(
                                "Failed to fetch season {} for TV show {}: {}",
                                season.season_number,
                                id,
                                e
                            );
                            seasons_with_episodes.push(season.clone());
                        }
                    }
                }
            }

            tv_details.seasons = seasons_with_episodes;
            Ok(tv_details)
        } else {
            self.get(&url).await
        }
    }

    /// Get detailed season information including episodes
    pub async fn get_season_details(&self, tv_id: i64, season_number: i32) -> Result<Season> {
        self.get(&format!("/tv/{}/season/{}", tv_id, season_number))
            .await
    }

    /// Get image URL for a given path and size
    /// Size examples: "w500", "w780", "original" for posters
    /// Call load_config() first to ensure config is loaded
    pub async fn image_url(&self, path: Option<&str>, size: &str) -> Result<Option<String>> {
        if path.is_none() {
            return Ok(None);
        }

        let config = self.load_config().await?;
        let base_url = &config.images.secure_base_url;
        let path = path.unwrap();

        Ok(Some(format!("{}{}{}", base_url, size, path)))
    }

    /// Generic GET request handler
    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", TMDB_API_BASE, path)
        };

        let response = self.client.get(&url).send().await?;

        // Handle different status codes
        match response.status() {
            StatusCode::OK => {
                let text = response.text().await?;
                serde_json::from_str(&text).map_err(|e| TmdbError::Json(e))
            }
            StatusCode::UNAUTHORIZED => Err(TmdbError::Authentication),
            StatusCode::NOT_FOUND => Err(TmdbError::NotFound(path.to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(TmdbError::RateLimit),
            status => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(TmdbError::Api {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = TmdbClient::new("test_token".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_invalid_token() {
        // Test with invalid characters that can't be in a header
        let result = TmdbClient::new("invalid\ntoken".to_string());
        assert!(result.is_err());
    }
}
