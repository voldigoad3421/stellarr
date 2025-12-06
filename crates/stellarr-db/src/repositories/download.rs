use crate::Database;
use stellarr_core::{Download, DownloadClient, Result};
use uuid::Uuid;

pub struct DownloadRepository {
    db: Database,
}

impl DownloadRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_client(&self, client: &DownloadClient) -> Result<DownloadClient> {
        todo!("Implement client creation")
    }

    pub async fn create_download(&self, download: &Download) -> Result<Download> {
        todo!("Implement download creation")
    }

    pub async fn get_download_by_id(&self, id: Uuid) -> Result<Option<Download>> {
        todo!("Implement get_download_by_id")
    }

    pub async fn get_active_downloads(&self) -> Result<Vec<Download>> {
        todo!("Implement get_active_downloads")
    }

    pub async fn update_download(&self, download: &Download) -> Result<Download> {
        todo!("Implement update_download")
    }
}
