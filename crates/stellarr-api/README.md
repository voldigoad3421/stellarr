# Stellarr API

Complete REST API implementation for the Stellarr media management system, built with Actix-web.

## Overview

This crate provides a production-ready HTTP API server for Stellarr, exposing endpoints for:
- Movie management (add, list, search, download)
- TV show management (add, list, episodes, search)
- TMDB metadata search
- Indexer configuration (Torznab/Newznab)
- Download management
- System monitoring and health checks

## Architecture

### Core Components

- **Server** (`lib.rs`): Actix-web HTTP server with CORS and logging
- **State** (`state.rs`): Shared application state with database and TMDB client
- **Errors** (`error.rs`): Comprehensive error handling with automatic HTTP status mapping
- **Routes** (`routes/`): Organized route modules for different domains

### Technology Stack

- **Web Framework**: Actix-web 4.x
- **Serialization**: Serde for JSON request/response
- **Database**: SQLx (via stellarr-db)
- **Metadata**: TMDB API client (via stellarr-providers)
- **Logging**: tracing + tracing-actix-web
- **Error Handling**: thiserror + anyhow

## API Endpoints

### Movies (`/api/movies`)
- `GET /` - List all movies
- `POST /` - Add movie from TMDB
- `GET /{id}` - Get movie details
- `DELETE /{id}` - Remove movie
- `POST /{id}/search` - Search indexers for movie
- `POST /{id}/download` - Download specific release

### TV Shows (`/api/tv`)
- `GET /` - List all TV shows
- `POST /` - Add TV show from TMDB
- `GET /{id}` - Get show with seasons/episodes
- `DELETE /{id}` - Remove show
- `POST /{id}/search` - Search for episodes

### Search (`/api/search`)
- `GET /movie?q=query` - Search TMDB movies
- `GET /tv?q=query` - Search TMDB TV shows

### Indexers (`/api/indexers`)
- `GET /` - List indexers
- `POST /` - Create indexer
- `GET /{id}` - Get indexer details
- `PUT /{id}` - Update indexer
- `DELETE /{id}` - Delete indexer
- `POST /{id}/test` - Test connection
- `GET /{id}/stats` - Get statistics

### Downloads (`/api/downloads`)
- `GET /` - List active downloads
- `GET /{id}` - Get download details
- `DELETE /{id}` - Cancel download
- `POST /{id}/retry` - Retry failed download

### Download Clients (`/api/download-clients`)
- `GET /` - List clients
- `POST /` - Create client
- `POST /{id}/test` - Test client connection

### System (`/api/system`)
- `GET /status` - System status
- `GET /stats` - Statistics
- `GET /health` - Health check
- `GET /logs` - System logs

## Usage

```rust
use stellarr_api::{start_server, AppState};
use stellarr_db::{Database, DatabaseConfig};
use stellarr_providers::TmdbClient;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Setup database
    let db_config = DatabaseConfig {
        database_type: "postgres".to_string(),
        database_url: "postgresql://user:pass@localhost/stellarr".to_string(),
    };
    let db = Database::connect(&db_config).await.unwrap();
    db.run_migrations().await.unwrap();
    
    // Setup TMDB client
    let tmdb = TmdbClient::new(
        std::env::var("TMDB_ACCESS_TOKEN").unwrap()
    ).unwrap();
    
    // Create application state
    let state = AppState::new(db, tmdb);
    
    // Start server
    start_server(state, "0.0.0.0:8080").await
}
```

## Error Handling

All endpoints return consistent error responses:

```json
{
  "error": "NOT_FOUND",
  "message": "Movie with id abc-123 not found",
  "details": null
}
```

Error types:
- `NOT_FOUND` (404)
- `BAD_REQUEST` (400)
- `VALIDATION_ERROR` (400)
- `CONFLICT` (409)
- `INTERNAL_ERROR` (500)

## Request/Response Format

All requests and responses use JSON. Example adding a movie:

**Request:**
```bash
curl -X POST http://localhost:8080/api/movies \
  -H "Content-Type: application/json" \
  -d '{
    "tmdb_id": 550,
    "quality_profile_id": "uuid-here",
    "monitored": true,
    "search_now": false
  }'
```

**Response:**
```json
{
  "id": "generated-uuid",
  "tmdb_id": 550,
  "title": "Fight Club",
  "year": 1999,
  "status": "missing",
  "monitored": true,
  ...
}
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the Server

```bash
TMDB_ACCESS_TOKEN=your-token cargo run
```

## Dependencies

This crate depends on:
- `stellarr-core`: Domain models and business logic
- `stellarr-db`: Database layer (SQLx with Postgres/SQLite)
- `stellarr-providers`: External service integrations (TMDB, indexers, download clients)

## Features Implemented

- Complete REST API with 35+ endpoints
- CORS support for web clients
- Request/response logging
- Comprehensive error handling
- TMDB metadata integration
- Type-safe request/response DTOs
- Async/await throughout
- Thread-safe shared state

## TODO

- Database repository integration (placeholder TODOs in handlers)
- Indexer search implementation
- Download client communication
- Background job system
- Pagination on list endpoints
- Filtering and sorting
- Authentication/authorization
- OpenAPI/Swagger documentation
- Rate limiting
- Request validation middleware

## License

See workspace LICENSE file.
