use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{MediaItem, Series, Episode, Season};
use crate::error::Result;

/// Repository trait for media item operations
#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn create(&self, item: &MediaItem) -> Result<MediaItem>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MediaItem>>;
    async fn find_by_tmdb_id(&self, tmdb_id: i64) -> Result<Option<MediaItem>>;
    async fn list(&self, offset: u32, limit: u32) -> Result<Vec<MediaItem>>;
    async fn update(&self, item: &MediaItem) -> Result<MediaItem>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Repository trait for TV series operations
#[async_trait]
pub trait SeriesRepository: Send + Sync {
    async fn create(&self, series: &Series) -> Result<Series>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Series>>;
    async fn find_by_tmdb_id(&self, tmdb_id: i64) -> Result<Option<Series>>;
    async fn list(&self, offset: u32, limit: u32) -> Result<Vec<Series>>;
    async fn update(&self, series: &Series) -> Result<Series>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn get_seasons(&self, series_id: Uuid) -> Result<Vec<Season>>;
    async fn get_episodes(&self, season_id: Uuid) -> Result<Vec<Episode>>;
}

/// Provider trait for external media metadata services (TMDB, etc.)
#[async_trait]
pub trait MetadataProvider: Send + Sync {
    async fn search_movies(&self, query: &str) -> Result<Vec<MediaItem>>;
    async fn search_tv_shows(&self, query: &str) -> Result<Vec<MediaItem>>;
    async fn get_movie(&self, tmdb_id: i64) -> Result<MediaItem>;
    async fn get_tv_show(&self, tmdb_id: i64) -> Result<MediaItem>;
}
