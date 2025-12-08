use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use stellarr_core::{MediaItem, MediaType, MediaStatus};
use crate::{AppState, ApiError};

// Request/Response DTOs

#[derive(Debug, Deserialize)]
pub struct AddMovieRequest {
    /// TMDB ID of the movie
    pub tmdb_id: i64,
    /// Quality profile ID to use
    pub quality_profile_id: Uuid,
    /// Whether to monitor this movie
    #[serde(default = "default_true")]
    pub monitored: bool,
    /// Whether to search for the movie immediately
    #[serde(default)]
    pub search_now: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct MovieResponse {
    pub id: Uuid,
    pub tmdb_id: i64,
    pub imdb_id: Option<String>,
    pub title: String,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub status: MediaStatus,
    pub quality_profile_id: Uuid,
    pub current_quality: Option<String>,
    pub path: Option<String>,
    pub monitored: bool,
    pub added_at: String,
    pub updated_at: String,
}

impl From<MediaItem> for MovieResponse {
    fn from(item: MediaItem) -> Self {
        Self {
            id: item.id,
            tmdb_id: item.tmdb_id,
            imdb_id: item.imdb_id,
            title: item.title,
            year: item.year,
            overview: item.overview,
            poster_path: item.poster_path,
            backdrop_path: item.backdrop_path,
            status: item.status,
            quality_profile_id: item.quality_profile_id,
            current_quality: item.current_quality,
            path: item.path.map(|p| p.to_string_lossy().to_string()),
            monitored: item.monitored,
            added_at: item.added_at.to_rfc3339(),
            updated_at: item.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MoviesListResponse {
    pub movies: Vec<MovieResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    /// Optional: only search for specific release
    pub quality: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadRequest {
    /// Release ID to download
    pub release_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub releases: Vec<ReleaseInfo>,
}

#[derive(Debug, Serialize)]
pub struct ReleaseInfo {
    pub id: Uuid,
    pub indexer_name: String,
    pub title: String,
    pub size: String,
    pub quality: String,
    pub seeders: Option<i32>,
    pub published_at: String,
}

// Route Handlers

/// GET /api/movies - List all movies
async fn list_movies(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    // TODO: Implement pagination and filtering
    // For now, return empty list with proper structure
    tracing::debug!("Listing all movies");
    
    // This would query the database in a real implementation
    let movies = Vec::new();
    
    Ok(HttpResponse::Ok().json(MoviesListResponse {
        movies,
        total: 0,
    }))
}

/// GET /api/movies/{id} - Get movie details
async fn get_movie(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    tracing::debug!("Getting movie with id: {}", id);
    
    // TODO: Query database for movie
    // For now, return not found
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Movie with id {} not found", id)))
}

/// POST /api/movies - Add movie from TMDB
async fn add_movie(
    state: web::Data<AppState>,
    req: web::Json<AddMovieRequest>,
) -> Result<impl Responder, ApiError> {
    tracing::info!("Adding movie from TMDB ID: {}", req.tmdb_id);
    
    // Fetch movie details from TMDB
    let tmdb_movie = state.tmdb.get_movie_details(req.tmdb_id).await?;
    
    // Create media item
    let mut movie = MediaItem::new(
        req.tmdb_id,
        tmdb_movie.title.clone(),
        MediaType::Movie,
        req.quality_profile_id,
    );
    
    // Populate additional fields from TMDB
    movie.year = tmdb_movie.release_date
        .and_then(|d| d.split('-').next().and_then(|y| y.parse().ok()));
    movie.overview = tmdb_movie.overview;
    movie.poster_path = tmdb_movie.poster_path;
    movie.backdrop_path = tmdb_movie.backdrop_path;
    
    // IMDb ID is directly on the Movie object from TMDB
    movie.imdb_id = tmdb_movie.imdb_id;
    
    movie.monitored = req.monitored;
    
    // TODO: Save to database
    // For now, just return the created movie
    
    tracing::info!("Added movie: {} ({})", movie.title, movie.tmdb_id);
    
    // If search_now is true, trigger a search
    if req.search_now {
        tracing::info!("Triggering immediate search for movie: {}", movie.title);
        // TODO: Trigger search in background
    }
    
    Ok(HttpResponse::Created().json(MovieResponse::from(movie)))
}

/// DELETE /api/movies/{id} - Remove movie
async fn delete_movie(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    tracing::info!("Deleting movie with id: {}", id);
    
    // TODO: Delete from database
    // For now, return 404
    Err::<HttpResponse, _>(ApiError::NotFound(format!("Movie with id {} not found", id)))
}

/// POST /api/movies/{id}/search - Search indexers for movie
async fn search_movie(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    query: web::Query<SearchRequest>,
) -> Result<impl Responder, ApiError> {
    tracing::info!("Searching for releases for movie: {}", id);
    
    // TODO: 
    // 1. Get movie from database
    // 2. Search all enabled indexers
    // 3. Score and filter releases
    // 4. Return results
    
    Ok(HttpResponse::Ok().json(SearchResponse {
        releases: Vec::new(),
    }))
}

/// POST /api/movies/{id}/download - Download specific release
async fn download_movie(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    req: web::Json<DownloadRequest>,
) -> Result<impl Responder, ApiError> {
    tracing::info!("Downloading release {} for movie {}", req.release_id, id);
    
    // TODO:
    // 1. Get movie from database
    // 2. Get release details
    // 3. Select appropriate download client
    // 4. Send download to client
    // 5. Track download in database
    
    Ok(HttpResponse::Accepted().json(serde_json::json!({
        "message": "Download queued",
        "release_id": req.release_id,
    })))
}

/// Configure movie routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/movies")
            .route("", web::get().to(list_movies))
            .route("", web::post().to(add_movie))
            .route("/{id}", web::get().to(get_movie))
            .route("/{id}", web::delete().to(delete_movie))
            .route("/{id}/search", web::post().to(search_movie))
            .route("/{id}/download", web::post().to(download_movie)),
    );
}

