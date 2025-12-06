# Stellarr

A unified media management application combining the best of Sonarr and Radarr into a single, modern solution.

## Features

- **Unified Interface**: Manage both movies and TV shows from a single application
- **Automatic Downloads**: Monitor and automatically download new releases
- **Quality Management**: Define quality profiles and upgrade existing media
- **Multiple Backends**: Support for various indexers and download clients
- **Modern Web UI**: Built with Leptos for a fast, reactive user experience
- **Flexible Database**: Support for both SQLite and PostgreSQL

## Architecture

Stellarr is built as a Rust workspace with multiple crates:

- **stellarr-core**: Domain models, business logic, and traits
- **stellarr-db**: Database layer with SQLx for migrations and queries
- **stellarr-api**: REST API built with Actix-web
- **stellarr-providers**: Integrations with external services (TMDB, indexers, download clients)
- **stellarr-web**: Frontend UI built with Leptos
- **stellarr**: Main binary that ties everything together

## Quick Start

### Prerequisites

- Rust 1.75+ (2024 stable)
- SQLite or PostgreSQL
- TMDB API key (for metadata)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/stellarr.git
cd stellarr

# Build the project
cargo build --release

# Run the application
./target/release/stellarr serve
```

### Configuration

Create a `.env` file or set environment variables:

```env
STELLARR_SERVER_HOST=127.0.0.1
STELLARR_SERVER_PORT=8080
STELLARR_DATABASE_TYPE=sqlite
STELLARR_DATABASE_URL=sqlite://stellarr.db
STELLARR_TMDB_API_KEY=your_tmdb_api_key_here
```

## Development

### Building

```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p stellarr-core

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Database Migrations

```bash
# Run migrations
cargo run -- migrate

# Create new migration (requires sqlx-cli)
sqlx migrate add create_movies_table
```

## Supported Integrations

### Metadata Providers
- TMDB (The Movie Database)

### Indexers
- Jackett (Torznab)
- Prowlarr

### Download Clients
- qBittorrent
- Transmission
- SABnzbd

## API Documentation

Once running, access the API at `http://localhost:8080/api/v1`

Key endpoints:
- `GET /health` - Health check
- `GET /api/v1/movies` - List movies
- `POST /api/v1/movies` - Add movie
- `GET /api/v1/tv` - List TV shows
- `POST /api/v1/tv` - Add TV show

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Acknowledgments

Inspired by the excellent work of:
- Sonarr team
- Radarr team
- The Rust community
