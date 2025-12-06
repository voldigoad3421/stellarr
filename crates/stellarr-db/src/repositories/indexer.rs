use crate::Database;
use stellarr_core::{Indexer, IndexerStats, Result};
use uuid::Uuid;

pub struct IndexerRepository {
    db: Database,
}

impl IndexerRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, indexer: &Indexer) -> Result<Indexer> {
        todo!("Implement indexer creation")
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Indexer>> {
        todo!("Implement get_by_id")
    }

    pub async fn get_all_enabled(&self) -> Result<Vec<Indexer>> {
        todo!("Implement get_all_enabled")
    }

    pub async fn update(&self, indexer: &Indexer) -> Result<Indexer> {
        todo!("Implement update")
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        todo!("Implement delete")
    }
}
