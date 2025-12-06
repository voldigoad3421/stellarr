//! TMDB (The Movie Database) metadata provider
//!
//! This module provides a client for interacting with the TMDB API v3.
//! It supports searching for movies and TV shows, fetching detailed information,
//! and retrieving image URLs with proper configuration.
//!
//! # Authentication
//!
//! The TMDB API requires an access token (Bearer token) for authentication.
//! You can obtain one from https://www.themoviedb.org/settings/api
//!
//! # Example
//!
//! ```no_run
//! use stellarr_providers::tmdb::TmdbClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = TmdbClient::new("your_access_token".to_string())?;
//!
//! // Search for movies
//! let results = client.search_movies("The Matrix", None).await?;
//!
//! // Get movie details
//! if let Some(movie) = results.results.first() {
//!     let details = client.get_movie_details(movie.id).await?;
//!     println!("Found: {}", details.title);
//! }
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod models;

pub use client::TmdbClient;
pub use error::{Result, TmdbError};
pub use models::{
    Configuration, Episode, ImageConfig, Movie, MovieSearchResponse, Season, TvDetails,
    TvSearchResponse, TvShow,
};
