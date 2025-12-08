use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use stellarr_core::{Indexer, IndexerProtocol, IndexerStats};
use crate::{AppState, ApiError};

#[derive(Debug, Deserialize)]
pub struct CreateIndexerRequest {
    pub name: String,
    pub protocol: IndexerProtocol,
    pub url: String,
    pub api_key: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_priority")]
    pub priority: i32,
    #[serde(default)]
    pub categories: Vec<i32>,
}

fn default_true() -> bool { true }
fn default_priority() -> i32 { 50 }

#[derive(Debug, Deserialize)]
pub struct UpdateIndexerRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub api_key: Option<String>,
    pub enabled: Option<bool>,
    pub priority: Option<i32>,
    pub categories: Option<Vec<i32>>,
}

#[derive(Debug, Serialize)]
pub struct IndexerResponse {
    pub id: Uuid,
    pub name: String,
    pub protocol: IndexerProtocol,
    pub url: String,
    pub enabled: bool,
    pub priority: i32,
    pub categories: Vec<i32>,
    pub capabilities: Option<serde_json::Value>,
    pub last_used_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Indexer> for IndexerResponse {
    fn from(indexer: Indexer) -> Self {
        Self {
            id: indexer.id,
            name: indexer.name,
            protocol: indexer.protocol,
            url: indexer.url,
            enabled: indexer.enabled,
            priority: indexer.priority,
            categories: indexer.categories,
            capabilities: indexer.capabilities,
            last_used_at: indexer.last_used_at.map(|dt| dt.to_rfc3339()),
            created_at: indexer.created_at.to_rfc3339(),
            updated_at: indexer.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IndexersListResponse {
    pub indexers: Vec<IndexerResponse>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct IndexerStatsResponse {
    pub indexer_id: Uuid,
    pub total_queries: i64,
    pub successful_queries: i64,
    pub failed_queries: i64,
    pub total_results: i64,
    pub grabbed_results: i64,
    pub success_rate: f64,
    pub grab_rate: f64,
    pub avg_response_time_ms: Option<i32>,
}

impl From<IndexerStats> for IndexerStatsResponse {
    fn from(stats: IndexerStats) -> Self {
        Self {
            indexer_id: stats.indexer_id,
            total_queries: stats.total_queries,
            successful_queries: stats.successful_queries,
            failed_queries: stats.failed_queries,
            total_results: stats.total_results,
            grabbed_results: stats.grabbed_results,
            success_rate: stats.success_rate(),
            grab_rate: stats.grab_rate(),
            avg_response_time_ms: stats.avg_response_time_ms,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TestIndexerResponse {
    pub success: bool,
    pub message: String,
    pub capabilities: Option<serde_json::Value>,
}

async fn list_indexers(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Listing all indexers");
    
    let indexers = Vec::new();
    
    Ok(HttpResponse::Ok().json(IndexersListResponse {
        indexers,
        total: 0,
    }))
}

async fn get_indexer(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Getting indexer with id: {}", id);
    
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Indexer with id {} not found", id)))
}

async fn create_indexer(state: web::Data<AppState>, req: web::Json<CreateIndexerRequest>) -> Result<impl Responder, ApiError> {
    tracing::info!("Creating indexer: {}", req.name);
    
    let mut indexer = Indexer::new(
        req.name.clone(),
        req.protocol,
        req.url.clone(),
        req.api_key.clone(),
    );
    
    indexer.enabled = req.enabled;
    indexer.priority = req.priority;
    
    if !req.categories.is_empty() {
        indexer.categories = req.categories.clone();
    }
    
    Ok(HttpResponse::Created().json(IndexerResponse::from(indexer)))
}

async fn update_indexer(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateIndexerRequest>,
) -> Result<impl Responder, ApiError> {
    tracing::info!("Updating indexer with id: {}", id);
    
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Indexer with id {} not found", id)))
}

async fn delete_indexer(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::info!("Deleting indexer with id: {}", id);
    
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Indexer with id {} not found", id)))
}

async fn test_indexer(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::info!("Testing indexer with id: {}", id);
    
    Ok(HttpResponse::Ok().json(TestIndexerResponse {
        success: false,
        message: "Not implemented".to_string(),
        capabilities: None,
    }))
}

async fn get_indexer_stats(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Getting stats for indexer: {}", id);
    
    let stats = IndexerStats::new(*id);
    
    Ok(HttpResponse::Ok().json(IndexerStatsResponse::from(stats)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/indexers")
            .route("", web::get().to(list_indexers))
            .route("", web::post().to(create_indexer))
            .route("/{id}", web::get().to(get_indexer))
            .route("/{id}", web::put().to(update_indexer))
            .route("/{id}", web::delete().to(delete_indexer))
            .route("/{id}/test", web::post().to(test_indexer))
            .route("/{id}/stats", web::get().to(get_indexer_stats)),
    );
}
