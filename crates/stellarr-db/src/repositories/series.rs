use crate::Database;
use stellarr_core::{Series, Season, Episode, Result};
use uuid::Uuid;

pub struct SeriesRepository {
    db: Database,
}

impl SeriesRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_series(&self, series: &Series) -> Result<Series> {
        todo!("Implement series creation")
    }

    pub async fn get_series_by_id(&self, id: Uuid) -> Result<Option<Series>> {
        todo!("Implement get_series_by_id")
    }

    pub async fn create_season(&self, season: &Season) -> Result<Season> {
        todo!("Implement season creation")
    }

    pub async fn create_episode(&self, episode: &Episode) -> Result<Episode> {
        todo!("Implement episode creation")
    }

    pub async fn get_episodes_for_season(&self, season_id: Uuid) -> Result<Vec<Episode>> {
        todo!("Implement get_episodes_for_season")
    }

    pub async fn get_missing_episodes(&self, series_id: Uuid) -> Result<Vec<Episode>> {
        todo!("Implement get_missing_episodes")
    }
}
