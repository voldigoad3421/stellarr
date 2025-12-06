use async_trait::async_trait;
use stellarr_core::{
    domain::TvShow,
    traits::TvShowRepository,
    Result, Error,
};
use uuid::Uuid;

use crate::Database;

/// SQLx-based TV show repository implementation
pub struct TvShowRepositoryImpl {
    db: Database,
}

impl TvShowRepositoryImpl {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TvShowRepository for TvShowRepositoryImpl {
    async fn create(&self, show: &TvShow) -> Result<TvShow> {
        // TODO: Implement TV show creation
        todo!("Implement TV show creation")
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TvShow>> {
        // TODO: Implement TV show lookup by ID
        todo!("Implement TV show lookup by ID")
    }

    async fn find_by_tmdb_id(&self, tmdb_id: i64) -> Result<Option<TvShow>> {
        // TODO: Implement TV show lookup by TMDB ID
        todo!("Implement TV show lookup by TMDB ID")
    }

    async fn list(&self, offset: u32, limit: u32) -> Result<Vec<TvShow>> {
        // TODO: Implement TV show listing with pagination
        todo!("Implement TV show listing")
    }

    async fn update(&self, show: &TvShow) -> Result<TvShow> {
        // TODO: Implement TV show update
        todo!("Implement TV show update")
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        // TODO: Implement TV show deletion
        todo!("Implement TV show deletion")
    }
}
