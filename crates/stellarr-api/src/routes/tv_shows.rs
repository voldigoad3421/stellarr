use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use stellarr_core::{Series, SeriesStatus, Season, SeasonStatus, Episode, EpisodeStatus};
use crate::{AppState, ApiError};

// Request/Response DTOs

#[derive(Debug, Deserialize)]
pub struct AddTvShowRequest {
    pub tmdb_id: i64,
    pub quality_profile_id: Uuid,
    #[serde(default = "default_true")]
    pub monitored: bool,
    #[serde(default)]
    pub monitor_seasons: Vec<i32>,
    #[serde(default)]
    pub search_now: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct TvShowResponse {
    pub id: Uuid,
    pub tmdb_id: i64,
    pub tvdb_id: Option<i64>,
    pub imdb_id: Option<String>,
    pub title: String,
    pub original_title: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub series_status: SeriesStatus,
    pub quality_profile_id: Uuid,
    pub path: Option<String>,
    pub monitored: bool,
    pub added_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<SeasonResponse>>,
}

#[derive(Debug, Serialize)]
pub struct SeasonResponse {
    pub id: Uuid,
    pub series_id: Uuid,
    pub season_number: i32,
    pub name: String,
    pub overview: Option<String>,
    pub air_date: Option<String>,
    pub poster_path: Option<String>,
    pub episode_count: i32,
    pub status: SeasonStatus,
    pub monitored: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes: Option<Vec<EpisodeResponse>>,
}

#[derive(Debug, Serialize)]
pub struct EpisodeResponse {
    pub id: Uuid,
    pub season_id: Uuid,
    pub series_id: Uuid,
    pub episode_number: i32,
    pub season_number: i32,
    pub title: String,
    pub overview: Option<String>,
    pub air_date: Option<String>,
    pub runtime: Option<i32>,
    pub still_path: Option<String>,
    pub status: EpisodeStatus,
    pub current_quality: Option<String>,
    pub file_path: Option<String>,
    pub monitored: bool,
    pub episode_code: String,
}

impl From<Series> for TvShowResponse {
    fn from(series: Series) -> Self {
        Self {
            id: series.id,
            tmdb_id: series.tmdb_id,
            tvdb_id: series.tvdb_id,
            imdb_id: series.imdb_id,
            title: series.title,
            original_title: series.original_title,
            year: series.year,
            overview: series.overview,
            poster_path: series.poster_path,
            backdrop_path: series.backdrop_path,
            series_status: series.series_status,
            quality_profile_id: series.quality_profile_id,
            path: series.path.map(|p| p.to_string_lossy().to_string()),
            monitored: series.monitored,
            added_at: series.added_at.to_rfc3339(),
            updated_at: series.updated_at.to_rfc3339(),
            seasons: None,
        }
    }
}

impl From<Episode> for EpisodeResponse {
    fn from(ep: Episode) -> Self {
        let episode_code = format!("S{:02}E{:02}", ep.season_number, ep.episode_number);
        Self {
            id: ep.id,
            season_id: ep.season_id,
            series_id: ep.series_id,
            episode_number: ep.episode_number,
            season_number: ep.season_number,
            title: ep.title,
            overview: ep.overview,
            air_date: ep.air_date.map(|d| d.to_string()),
            runtime: ep.runtime,
            still_path: ep.still_path,
            status: ep.status,
            current_quality: ep.current_quality,
            file_path: ep.file_path.map(|p| p.to_string_lossy().to_string()),
            monitored: ep.monitored,
            episode_code,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TvShowsListResponse {
    pub tv_shows: Vec<TvShowResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct SearchEpisodesRequest {
    pub season: Option<i32>,
    pub episode: Option<i32>,
}

async fn list_tv_shows(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Listing all TV shows");
    let tv_shows = Vec::new();
    Ok(HttpResponse::Ok().json(TvShowsListResponse { tv_shows, total: 0 }))
}

async fn get_tv_show(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::debug!("Getting TV show with id: {}", id);
    Err::<HttpResponse, _>(ApiError::NotFound(format!("TV show with id {} not found", id)))
}

async fn add_tv_show(state: web::Data<AppState>, req: web::Json<AddTvShowRequest>) -> Result<impl Responder, ApiError> {
    tracing::info!("Adding TV show from TMDB ID: {}", req.tmdb_id);
    
    let tmdb_show = state.tmdb.get_tv_details(req.tmdb_id, true).await?;
    
    let mut series = Series::new(req.tmdb_id, tmdb_show.name.clone(), req.quality_profile_id);
    series.original_title = Some(tmdb_show.original_name);
    series.overview = tmdb_show.overview;
    series.poster_path = tmdb_show.poster_path;
    series.backdrop_path = tmdb_show.backdrop_path;
    series.monitored = req.monitored;
    
    if let Some(date) = tmdb_show.first_air_date {
        series.year = date.split('-').next().and_then(|y| y.parse().ok());
    }
    
    series.series_status = match tmdb_show.status.as_deref() {
        Some("Returning Series") => SeriesStatus::Continuing,
        Some("Ended") => SeriesStatus::Ended,
        Some("Canceled") => SeriesStatus::Canceled,
        Some("In Production") => SeriesStatus::Upcoming,
        _ => SeriesStatus::Continuing,
    };
    
    if let Some(external_ids) = tmdb_show.external_ids {
        series.imdb_id = external_ids.imdb_id;
        series.tvdb_id = external_ids.tvdb_id;
    }
    
    tracing::info!("Added TV show: {} ({})", series.title, series.tmdb_id);
    
    if req.search_now {
        tracing::info!("Triggering immediate search for TV show: {}", series.title);
    }
    
    Ok(HttpResponse::Created().json(TvShowResponse::from(series)))
}

async fn delete_tv_show(state: web::Data<AppState>, id: web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    tracing::info!("Deleting TV show with id: {}", id);
    Err::<HttpResponse, _>(ApiError::NotFound(format!("TV show with id {} not found", id)))
}

async fn search_tv_show(state: web::Data<AppState>, id: web::Path<Uuid>, query: web::Query<SearchEpisodesRequest>) -> Result<impl Responder, ApiError> {
    tracing::info!("Searching for episodes for TV show: {}", id);
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Search initiated",
        "season": query.season,
        "episode": query.episode,
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tv")
            .route("", web::get().to(list_tv_shows))
            .route("", web::post().to(add_tv_show))
            .route("/{id}", web::get().to(get_tv_show))
            .route("/{id}", web::delete().to(delete_tv_show))
            .route("/{id}/search", web::post().to(search_tv_show)),
    );
}

