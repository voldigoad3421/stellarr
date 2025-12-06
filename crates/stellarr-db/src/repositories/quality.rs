use crate::Database;
use stellarr_core::{QualityProfile, Result};
use uuid::Uuid;

pub struct QualityProfileRepository {
    db: Database,
}

impl QualityProfileRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, profile: &QualityProfile) -> Result<QualityProfile> {
        todo!("Implement profile creation")
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<QualityProfile>> {
        todo!("Implement get_by_id")
    }

    pub async fn get_all(&self) -> Result<Vec<QualityProfile>> {
        todo!("Implement get_all")
    }

    pub async fn update(&self, profile: &QualityProfile) -> Result<QualityProfile> {
        todo!("Implement update")
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        todo!("Implement delete")
    }
}
