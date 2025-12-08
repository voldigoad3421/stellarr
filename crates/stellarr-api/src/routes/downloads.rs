use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use stellarr_core::{Download, DownloadStatus, DownloadClient, DownloadClientType};
use crate::{AppState, ApiError};

#[derive(Debug, Serialize)]
pub struct DownloadResponse {
    pub id: Uuid,
    pub client_id: Uuid,
    pub media_item_id: Option<Uuid>,
    pub episode_id: Option<Uuid>,
    pub title: String,
    pub download_id: String,
    pub status: DownloadStatus,
    pub progress: f64,
    pub size_bytes: Option<i64>,
    pub downloaded_bytes: Option<i64>,
    pub download_speed: Option<i64>,
    pub eta_seconds: Option<i64>,
    pub error_message: Option<String>,
    pub output_path: Option<String>,
    pub quality: Option<String>,
    pub added_at: String,
    pub completed_at: Option<String>,
    pub updated_at: String,
}

impl From<Download> for DownloadResponse {
    fn from(dl: Download) -> Self {
        Self {
            id: dl.id,
            client_id: dl.client_id,
            media_item_id: dl.media_item_id,
            episode_id: dl.episode_id,
            title: dl.title,
            download_id: dl.download_id,
            status: dl.status,
            progress: dl.progress,
            size_bytes: dl.size_bytes,
            downloaded_bytes: dl.downloaded_bytes,
            download_speed: dl.download_speed,
            eta_seconds: dl.eta_seconds,
            error_message: dl.error_message,
            output_path: dl.output_path.map(|p| p.to_string_lossy().to_string()),
            quality: dl.quality,
            added_at: dl.added_at.to_rfc3339(),
            completed_at: dl.completed_at.map(|dt| dt.to_rfc3339()),
            updated_at: dl.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DownloadsListResponse {
    pub downloads: Vec<DownloadResponse>,
    pub active: usize,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct DownloadClientResponse {
    pub id: Uuid,
    pub name: String,
    pub client_type: DownloadClientType,
    pub host: String,
    pub port: u16,
    pub enabled: bool,
    pub priority: i32,
}

impl From<DownloadClient> for DownloadClientResponse {
    fn from(client: DownloadClient) -> Self {
        Self {
            id: client.id,
            name: client.name,
            client_type: client.client_type,
            host: client.host,
            port: client.port,
            enabled: client.enabled,
            priority: client.priority,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DownloadClientsListResponse {
    pub clients: Vec<DownloadClientResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct CreateDownloadClientRequest {
    pub name: String,
    pub client_type: DownloadClientType,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub api_key: Option<String>,
    #[serde(default)]
    pub use_ssl: bool,
    pub url_base: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_priority")]
    pub priority: i32,
}

fn default_true() -> bool { true }
fn default_priority() -> i32 { 50 }

async fn list_downloads(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Listing all downloads");
    
    let downloads = Vec::new();
    
    Ok(HttpResponse::Ok().json(DownloadsListResponse {
        downloads,
        active: 0,
        total: 0,
    }))
}

async fn get_download(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Getting download with id: {}", id);
    
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Download with id {} not found", id)))
}

async fn cancel_download(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::info!("Cancelling download with id: {}", id);
    
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Download with id {} not found", id)))
}

async fn retry_download(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::info!("Retrying download with id: {}", id);
    
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Download with id {} not found", id)))
}

async fn list_download_clients(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Listing all download clients");
    
    let clients = Vec::new();
    
    Ok(HttpResponse::Ok().json(DownloadClientsListResponse {
        clients,
        total: 0,
    }))
}

async fn create_download_client(
    state: web::Data<AppState>,
    req: web::Json<CreateDownloadClientRequest>,
) -> Result<impl Responder, ApiError> {
    tracing::info!("Creating download client: {}", req.name);
    
    let mut client = DownloadClient::new(
        req.name.clone(),
        req.client_type,
        req.host.clone(),
        req.port,
    );
    
    client.username = req.username.clone();
    client.password = req.password.clone();
    client.api_key = req.api_key.clone();
    client.use_ssl = req.use_ssl;
    client.url_base = req.url_base.clone();
    client.enabled = req.enabled;
    client.priority = req.priority;
    
    Ok(HttpResponse::Created().json(DownloadClientResponse::from(client)))
}

async fn test_download_client(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::info!("Testing download client with id: {}", id);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": false,
        "message": "Not implemented"
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/downloads")
            .route("", web::get().to(list_downloads))
            .route("/{id}", web::get().to(get_download))
            .route("/{id}", web::delete().to(cancel_download))
            .route("/{id}/retry", web::post().to(retry_download))
    )
    .service(
        web::scope("/download-clients")
            .route("", web::get().to(list_download_clients))
            .route("", web::post().to(create_download_client))
            .route("/{id}/test", web::post().to(test_download_client))
    );
}
