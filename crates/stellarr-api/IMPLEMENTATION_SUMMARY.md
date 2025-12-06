# Stellarr API Implementation Summary

Complete REST API implementation for Stellarr media management system built with Actix-web.

## Project Structure

```
stellarr-api/
├── src/
│   ├── lib.rs              # Server setup with CORS, logging
│   ├── state.rs            # AppState with DB pool and TMDB client
│   ├── error.rs            # Comprehensive error handling
│   ├── routes.rs           # Route configuration (mod file)
│   ├── handlers.rs         # (existing)
│   ├── middleware.rs       # (existing)
│   └── routes/
│       ├── health.rs       # Health check endpoint
│       ├── movies.rs       # Movie management endpoints
│       ├── tv_shows.rs     # TV show management endpoints
│       ├── search.rs       # TMDB search endpoints
│       ├── indexers.rs     # Indexer management endpoints
│       ├── downloads.rs    # Download management endpoints
│       └── system.rs       # System status endpoints
└── Cargo.toml
```

## Implemented Files

### Core Files (Modified/Created)

1. **src/state.rs** (24 lines)
   - Database pool integration
   - TMDB client integration
   - Thread-safe Arc wrapping

2. **src/error.rs** (125 lines)
   - ApiError enum with 7 error types
   - Automatic conversion from stellarr-core errors
   - Database error conversion (including constraint violations)
   - TMDB error conversion
   - Consistent JSON error responses

3. **src/routes.rs** (20 lines)
   - Configures all route modules
   - Single /api scope for all endpoints

### Route Modules

4. **src/routes/movies.rs** (240 lines)
   - GET /api/movies - List all movies
   - GET /api/movies/{id} - Get movie details
   - POST /api/movies - Add movie from TMDB ID
   - DELETE /api/movies/{id} - Remove movie
   - POST /api/movies/{id}/search - Search indexers
   - POST /api/movies/{id}/download - Download specific release
   - Full DTOs: AddMovieRequest, MovieResponse, MoviesListResponse

5. **src/routes/tv_shows.rs** (213 lines)
   - GET /api/tv - List all TV shows
   - GET /api/tv/{id} - Get show with seasons/episodes
   - POST /api/tv - Add show from TMDB
   - DELETE /api/tv/{id} - Remove show
   - POST /api/tv/{id}/search - Search for episodes
   - Full DTOs: TvShowResponse, SeasonResponse, EpisodeResponse

6. **src/routes/search.rs** (134 lines)
   - GET /api/search/movie?q=query - Search TMDB movies
   - GET /api/search/tv?q=query - Search TMDB TV shows
   - Pagination support
   - DTOs: MovieSearchResult, TvSearchResult

7. **src/routes/indexers.rs** (191 lines)
   - GET /api/indexers - List all indexers
   - GET /api/indexers/{id} - Get indexer details
   - POST /api/indexers - Create indexer
   - PUT /api/indexers/{id} - Update indexer
   - DELETE /api/indexers/{id} - Delete indexer
   - POST /api/indexers/{id}/test - Test connection
   - GET /api/indexers/{id}/stats - Get statistics
   - DTOs: IndexerResponse, IndexerStatsResponse

8. **src/routes/downloads.rs** (201 lines)
   - GET /api/downloads - List active downloads
   - GET /api/downloads/{id} - Get download details
   - DELETE /api/downloads/{id} - Cancel download
   - POST /api/downloads/{id}/retry - Retry failed download
   - GET /api/download-clients - List clients
   - POST /api/download-clients - Create client
   - POST /api/download-clients/{id}/test - Test client
   - DTOs: DownloadResponse, DownloadClientResponse

9. **src/routes/system.rs** (149 lines)
   - GET /api/system/status - System status
   - GET /api/system/stats - Statistics
   - GET /api/system/health - Health check
   - GET /api/system/logs - System logs
   - DTOs: SystemStatusResponse, SystemStatsResponse, HealthCheckResponse

## Features Implemented

### Server Setup (lib.rs)
- Actix-web HTTP server
- CORS configuration (allow any origin)
- Request logging with tracing-actix-web
- Shared application state

### Error Handling (error.rs)
- 7 error types: NotFound, BadRequest, InternalError, DatabaseError, TmdbError, ValidationError, Conflict
- Automatic HTTP status code mapping
- JSON error responses with error type and message
- Conversions from:
  - stellarr_core::Error
  - sqlx::Error
  - TMDB errors
  - anyhow::Error
  - uuid::Error

### Request/Response DTOs
All endpoints use proper serializable DTOs with:
- serde Serialize/Deserialize
- Optional fields with skip_serializing_if
- Default values where appropriate
- Type conversions from domain models

### Integration Points
- stellarr-core domain models (MediaItem, Series, Episode, etc.)
- stellarr-db Database enum (Postgres/SQLite)
- stellarr-providers TmdbClient for metadata
- UUID for identifiers
- Proper timestamp formatting (RFC3339)

## Current State

### Fully Implemented
- All route handlers defined with proper signatures
- Complete DTO structures for all endpoints
- Error handling for all routes
- TMDB integration for movie/TV metadata
- Logging statements for all operations

### Placeholder TODOs
Routes are fully structured but contain TODO comments for:
- Database queries (integration with stellarr-db repositories)
- Indexer search implementation
- Download client communication
- Background job triggers

## Statistics

- Total route modules: 7
- Total endpoints: ~35+
- Total lines of code: ~1,147 (routes only)
- Error types: 7
- DTO types: 20+

## Next Steps

To complete the implementation:

1. Implement database repository methods in stellarr-db
2. Integrate indexer search functionality
3. Implement download client communication
4. Add background job system for searches/downloads
5. Add pagination to list endpoints
6. Add filtering/sorting to list endpoints
7. Implement authentication/authorization
8. Add OpenAPI/Swagger documentation generation

## Usage Example

```rust
use stellarr_api::{start_server, AppState};
use stellarr_db::{Database, DatabaseConfig};
use stellarr_providers::TmdbClient;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let db_config = DatabaseConfig {
        database_type: "sqlite".to_string(),
        database_url: "sqlite://stellarr.db".to_string(),
    };
    let db = Database::connect(&db_config).await.unwrap();
    
    // Initialize TMDB client
    let tmdb = TmdbClient::new("your-tmdb-token".to_string()).unwrap();
    
    // Create app state
    let state = AppState::new(db, tmdb);
    
    // Start server
    start_server(state, "127.0.0.1:8080").await
}
```

## API Endpoints Summary

### Movies
- List, Get, Add, Delete, Search, Download

### TV Shows  
- List, Get, Add, Delete, Search episodes

### Search
- Search TMDB for movies and TV shows

### Indexers
- CRUD operations, test connection, view stats

### Downloads
- List, Get, Cancel, Retry downloads
- Manage download clients

### System
- Status, statistics, health check, logs

All endpoints return consistent JSON responses with proper error handling.
