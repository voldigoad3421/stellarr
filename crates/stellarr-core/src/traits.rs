use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Movie, TvShow, Episode, Season};
use crate::error::Result;

/// Repository trait for movie operations
#[async_trait]
pub trait MovieRepository: Send + Sync {
    async fn create(&self, movie: &Movie) -> Result<Movie>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Movie>>;
    async fn find_by_tmdb_id(&self, tmdb_id: i64) -> Result<Option<Movie>>;
    async fn list(&self, offset: u32, limit: u32) -> Result<Vec<Movie>>;
    async fn update(&self, movie: &Movie) -> Result<Movie>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Repository trait for TV show operations
#[async_trait]
pub trait TvShowRepository: Send + Sync {
    async fn create(&self, show: &TvShow) -> Result<TvShow>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TvShow>>;
    async fn find_by_tmdb_id(&self, tmdb_id: i64) -> Result<Option<TvShow>>;
    async fn list(&self, offset: u32, limit: u32) -> Result<Vec<TvShow>>;
    async fn update(&self, show: &TvShow) -> Result<TvShow>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Provider trait for external media metadata services (TMDB, etc.)
#[async_trait]
pub trait MetadataProvider: Send + Sync {
    async fn search_movies(&self, query: &str) -> Result<Vec<Movie>>;
    async fn search_tv_shows(&self, query: &str) -> Result<Vec<TvShow>>;
    async fn get_movie(&self, tmdb_id: i64) -> Result<Movie>;
    async fn get_tv_show(&self, tmdb_id: i64) -> Result<TvShow>;
}
