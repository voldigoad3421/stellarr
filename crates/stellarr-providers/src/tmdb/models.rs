use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// TMDB API configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub images: ImageConfig,
}

/// Image configuration from TMDB
#[derive(Debug, Clone, Deserialize)]
pub struct ImageConfig {
    pub base_url: String,
    pub secure_base_url: String,
    pub backdrop_sizes: Vec<String>,
    pub logo_sizes: Vec<String>,
    pub poster_sizes: Vec<String>,
    pub profile_sizes: Vec<String>,
    pub still_sizes: Vec<String>,
}

/// Generic search response wrapper
#[derive(Debug, Deserialize)]
pub struct SearchResponse<T> {
    pub page: i32,
    pub results: Vec<T>,
    #[serde(default)]
    pub total_pages: i32,
    #[serde(default)]
    pub total_results: i32,
}

/// Movie search response
pub type MovieSearchResponse = SearchResponse<Movie>;

/// TV show search response
pub type TvSearchResponse = SearchResponse<TvShow>;

/// Movie from TMDB API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub release_date: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub genre_ids: Vec<i32>,
    #[serde(default)]
    pub genres: Vec<Genre>,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub popularity: f64,
    pub runtime: Option<i32>,
    pub imdb_id: Option<String>,
}

/// TV show from TMDB API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TvShow {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub overview: Option<String>,
    pub first_air_date: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub genre_ids: Vec<i32>,
    #[serde(default)]
    pub genres: Vec<Genre>,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub popularity: f64,
}

/// TV show details with seasons
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TvDetails {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub overview: Option<String>,
    pub first_air_date: Option<String>,
    pub last_air_date: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub genres: Vec<Genre>,
    #[serde(default)]
    pub vote_average: f64,
    pub status: Option<String>,
    #[serde(default)]
    pub seasons: Vec<Season>,
    #[serde(default)]
    pub number_of_seasons: i32,
    #[serde(default)]
    pub number_of_episodes: i32,
    pub external_ids: Option<ExternalIds>,
}

/// Season information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Season {
    pub id: i64,
    pub season_number: i32,
    pub name: String,
    pub overview: Option<String>,
    pub air_date: Option<String>,
    pub poster_path: Option<String>,
    #[serde(default)]
    pub episode_count: i32,
    #[serde(default)]
    pub episodes: Vec<Episode>,
}

/// Episode information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Episode {
    pub id: i64,
    pub episode_number: i32,
    pub season_number: i32,
    pub name: String,
    pub overview: Option<String>,
    pub air_date: Option<String>,
    pub runtime: Option<i32>,
    pub still_path: Option<String>,
    #[serde(default)]
    pub vote_average: f64,
}

/// Genre information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}

/// External IDs for a show/movie
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalIds {
    pub imdb_id: Option<String>,
    pub tvdb_id: Option<i64>,
}

impl Movie {
    /// Parse release date string to NaiveDate
    pub fn parse_release_date(&self) -> Option<NaiveDate> {
        self.release_date
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    }
}

impl TvShow {
    /// Parse first air date string to NaiveDate
    pub fn parse_first_air_date(&self) -> Option<NaiveDate> {
        self.first_air_date
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    }
}

impl TvDetails {
    /// Parse first air date string to NaiveDate
    pub fn parse_first_air_date(&self) -> Option<NaiveDate> {
        self.first_air_date
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    }
}

impl Season {
    /// Parse air date string to NaiveDate
    pub fn parse_air_date(&self) -> Option<NaiveDate> {
        self.air_date
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    }
}

impl Episode {
    /// Parse air date string to NaiveDate
    pub fn parse_air_date(&self) -> Option<NaiveDate> {
        self.air_date
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    }
}

