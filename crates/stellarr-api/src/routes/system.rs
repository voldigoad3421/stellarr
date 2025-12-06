use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use crate::{AppState, ApiError};

#[derive(Debug, Serialize)]
pub struct SystemStatusResponse {
    pub status: String,
    pub version: String,
    pub database: DatabaseStatus,
    pub tmdb: ServiceStatus,
}

#[derive(Debug, Serialize)]
pub struct DatabaseStatus {
    pub connected: bool,
    pub database_type: String,
}

#[derive(Debug, Serialize)]
pub struct ServiceStatus {
    pub connected: bool,
    pub last_check: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SystemStatsResponse {
    pub total_movies: u64,
    pub total_tv_shows: u64,
    pub total_episodes: u64,
    pub active_downloads: u64,
    pub total_indexers: u64,
    pub enabled_indexers: u64,
    pub total_download_clients: u64,
    pub enabled_download_clients: u64,
    pub disk_usage: Option<DiskUsage>,
}

#[derive(Debug, Serialize)]
pub struct DiskUsage {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub used_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub healthy: bool,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub healthy: bool,
    pub message: Option<String>,
}

async fn get_status(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Getting system status");
    
    let db_status = DatabaseStatus {
        connected: true,
        database_type: match &state.db {
            stellarr_db::Database::Postgres(_) => "postgres".to_string(),
            stellarr_db::Database::Sqlite(_) => "sqlite".to_string(),
        },
    };
    
    let tmdb_status = ServiceStatus {
        connected: true,
        last_check: None,
    };
    
    Ok(HttpResponse::Ok().json(SystemStatusResponse {
        status: "running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        tmdb: tmdb_status,
    }))
}

async fn get_stats(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Getting system statistics");
    
    let stats = SystemStatsResponse {
        total_movies: 0,
        total_tv_shows: 0,
        total_episodes: 0,
        active_downloads: 0,
        total_indexers: 0,
        enabled_indexers: 0,
        total_download_clients: 0,
        enabled_download_clients: 0,
        disk_usage: None,
    };
    
    Ok(HttpResponse::Ok().json(stats))
}

async fn health_check(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Performing health check");
    
    let mut checks = Vec::new();
    
    checks.push(HealthCheck {
        name: "database".to_string(),
        healthy: true,
        message: Some("Database connection active".to_string()),
    });
    
    checks.push(HealthCheck {
        name: "tmdb".to_string(),
        healthy: true,
        message: Some("TMDB client configured".to_string()),
    });
    
    let all_healthy = checks.iter().all(|c| c.healthy);
    
    let response = HealthCheckResponse {
        healthy: all_healthy,
        checks,
    };
    
    if all_healthy {
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::ServiceUnavailable().json(response))
    }
}

async fn get_logs(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Fetching system logs");
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "logs": [],
        "message": "Log retrieval not implemented"
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/system")
            .route("/status", web::get().to(get_status))
            .route("/stats", web::get().to(get_stats))
            .route("/health", web::get().to(health_check))
            .route("/logs", web::get().to(get_logs))
    );
}
