use gloo_net::http::Request;
use serde::de::DeserializeOwned;

const API_BASE: &str = "/api/v1";

/// API client for communicating with Stellarr backend
pub struct ApiClient;

impl ApiClient {
    /// Perform a GET request to the API
    pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
        let url = format!("{}{}", API_BASE, path);
        Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<T>()
            .await
            .map_err(|e| e.to_string())
    }

    /// Perform a POST request to the API
    pub async fn post<T: DeserializeOwned>(path: &str, body: impl serde::Serialize) -> Result<T, String> {
        let url = format!("{}{}", API_BASE, path);
        Request::post(&url)
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<T>()
            .await
            .map_err(|e| e.to_string())
    }

    /// Perform a PUT request to the API
    pub async fn put<T: DeserializeOwned>(path: &str, body: impl serde::Serialize) -> Result<T, String> {
        let url = format!("{}{}", API_BASE, path);
        Request::put(&url)
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<T>()
            .await
            .map_err(|e| e.to_string())
    }

    /// Perform a DELETE request to the API
    pub async fn delete<T: DeserializeOwned>(path: &str) -> Result<T, String> {
        let url = format!("{}{}", API_BASE, path);
        Request::delete(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<T>()
            .await
            .map_err(|e| e.to_string())
    }

    /// Perform a DELETE request with no response body
    pub async fn delete_no_response(path: &str) -> Result<(), String> {
        let url = format!("{}{}", API_BASE, path);
        Request::delete(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// Convenience methods for common operations
impl ApiClient {
    /// Fetch all movies
    pub async fn fetch_movies() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/movies").await
    }

    /// Fetch all TV series
    pub async fn fetch_series() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/series").await
    }

    /// Search for movies on TMDB
    pub async fn search_movies(query: &str) -> Result<Vec<serde_json::Value>, String> {
        Self::get(&format!("/search/movie?q={}", query)).await
    }

    /// Search for TV shows on TMDB
    pub async fn search_tv_shows(query: &str) -> Result<Vec<serde_json::Value>, String> {
        Self::get(&format!("/search/tv?q={}", query)).await
    }

    /// Add a movie to the library
    pub async fn add_movie(tmdb_id: i64, quality_profile_id: Option<String>) -> Result<serde_json::Value, String> {
        let body = serde_json::json!({
            "tmdb_id": tmdb_id,
            "quality_profile_id": quality_profile_id,
        });
        Self::post("/movies", body).await
    }

    /// Add a TV series to the library
    pub async fn add_series(tmdb_id: i64, quality_profile_id: Option<String>) -> Result<serde_json::Value, String> {
        let body = serde_json::json!({
            "tmdb_id": tmdb_id,
            "quality_profile_id": quality_profile_id,
        });
        Self::post("/series", body).await
    }

    /// Fetch active downloads
    pub async fn fetch_active_downloads() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/downloads/active").await
    }

    /// Fetch download history
    pub async fn fetch_download_history() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/downloads/history").await
    }

    /// Fetch dashboard statistics
    pub async fn fetch_stats() -> Result<serde_json::Value, String> {
        Self::get("/stats").await
    }

    /// Fetch recent activity
    pub async fn fetch_recent_activity() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/activity/recent").await
    }

    /// Fetch indexers
    pub async fn fetch_indexers() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/indexers").await
    }

    /// Fetch download clients
    pub async fn fetch_download_clients() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/download-clients").await
    }

    /// Fetch quality profiles
    pub async fn fetch_quality_profiles() -> Result<Vec<serde_json::Value>, String> {
        Self::get("/quality-profiles").await
    }

    /// Trigger a search for a movie
    pub async fn trigger_movie_search(movie_id: &str) -> Result<(), String> {
        Self::post::<serde_json::Value>(&format!("/movies/{}/search", movie_id), serde_json::json!({}))
            .await?;
        Ok(())
    }

    /// Trigger a search for a series
    pub async fn trigger_series_search(series_id: &str) -> Result<(), String> {
        Self::post::<serde_json::Value>(&format!("/series/{}/search", series_id), serde_json::json!({}))
            .await?;
        Ok(())
    }

    /// Delete a movie from the library
    pub async fn delete_movie(movie_id: &str) -> Result<(), String> {
        Self::delete_no_response(&format!("/movies/{}", movie_id)).await
    }

    /// Delete a series from the library
    pub async fn delete_series(series_id: &str) -> Result<(), String> {
        Self::delete_no_response(&format!("/series/{}", series_id)).await
    }
}
