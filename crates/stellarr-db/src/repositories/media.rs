use crate::Database;
use stellarr_core::{MediaItem, MediaType, Result};
use uuid::Uuid;

pub struct MediaRepository {
    db: Database,
}

impl MediaRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, media: &MediaItem) -> Result<MediaItem> {
        todo!("Implement media creation")
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<MediaItem>> {
        todo!("Implement get_by_id")
    }

    pub async fn get_by_tmdb_id(&self, tmdb_id: i64) -> Result<Option<MediaItem>> {
        todo!("Implement get_by_tmdb_id")
    }

    pub async fn get_all(&self, media_type: Option<MediaType>) -> Result<Vec<MediaItem>> {
        todo!("Implement get_all")
    }

    pub async fn update(&self, media: &MediaItem) -> Result<MediaItem> {
        todo!("Implement update")
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        todo!("Implement delete")
    }
}
