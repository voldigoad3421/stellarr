use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::{AppState, ApiError};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_page")]
    pub page: i32,
}

fn default_page() -> i32 {
    1
}

#[derive(Debug, Serialize)]
pub struct MovieSearchResult {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub release_date: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_average: f64,
}

#[derive(Debug, Serialize)]
pub struct TvSearchResult {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub first_air_date: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_average: f64,
}

#[derive(Debug, Serialize)]
pub struct MovieSearchResponse {
    pub results: Vec<MovieSearchResult>,
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(Debug, Serialize)]
pub struct TvSearchResponse {
    pub results: Vec<TvSearchResult>,
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
}

async fn search_movies(state: web::Data<AppState>, query: web::Query<SearchQuery>) -> Result<impl Responder, ApiError> {
    tracing::info!("Searching TMDB for movies: {} (page {})", query.q, query.page);
    
    let tmdb_response = state.tmdb.search_movies(&query.q, Some(query.page as u32)).await?;
    
    let results: Vec<MovieSearchResult> = tmdb_response.results.into_iter().map(|movie| {
        let year = movie.release_date.as_ref()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse().ok());
        
        MovieSearchResult {
            id: movie.id,
            title: movie.title.clone(),
            original_title: movie.original_title,
            release_date: movie.release_date,
            year,
            overview: movie.overview,
            poster_path: movie.poster_path,
            backdrop_path: movie.backdrop_path,
            popularity: movie.popularity,
            vote_average: movie.vote_average,
        }
    }).collect();
    
    Ok(HttpResponse::Ok().json(MovieSearchResponse {
        results,
        page: tmdb_response.page,
        total_pages: tmdb_response.total_pages,
        total_results: tmdb_response.total_results,
    }))
}

async fn search_tv(state: web::Data<AppState>, query: web::Query<SearchQuery>) -> Result<impl Responder, ApiError> {
    tracing::info!("Searching TMDB for TV shows: {} (page {})", query.q, query.page);
    
    let tmdb_response = state.tmdb.search_tv(&query.q, Some(query.page as u32)).await?;
    
    let results: Vec<TvSearchResult> = tmdb_response.results.into_iter().map(|show| {
        let year = show.first_air_date.as_ref()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse().ok());
        
        TvSearchResult {
            id: show.id,
            name: show.name.clone(),
            original_name: show.original_name,
            first_air_date: show.first_air_date,
            year,
            overview: show.overview,
            poster_path: show.poster_path,
            backdrop_path: show.backdrop_path,
            popularity: show.popularity,
            vote_average: show.vote_average,
        }
    }).collect();
    
    Ok(HttpResponse::Ok().json(TvSearchResponse {
        results,
        page: tmdb_response.page,
        total_pages: tmdb_response.total_pages,
        total_results: tmdb_response.total_results,
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/search")
            .route("/movie", web::get().to(search_movies))
            .route("/tv", web::get().to(search_tv)),
    );
}


