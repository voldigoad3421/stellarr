# Stellarr - Project Context for Claude

## What This Is
Stellarr is a unified media management app combining Sonarr (TV) + Radarr (Movies) into one modern Rust application. Think "arr apps but better and combined."

## Tech Stack
- **Backend**: Rust + Actix-web
- **Frontend**: Leptos (full-stack Rust/WASM)
- **Database**: SQLx (SQLite default, PostgreSQL optional)
- **Metadata**: TMDB API
- **Indexers**: Torznab/Newznab (Prowlarr/Jackett compatible)
- **Downloads**: qBittorrent

## Project Structure
```
stellarr/
├── stellarr/              # Main binary
├── crates/
│   ├── stellarr-core/     # Domain models, quality system, scoring
│   ├── stellarr-api/      # REST API (35+ endpoints)
│   ├── stellarr-db/       # Database layer, migrations
│   ├── stellarr-providers/# TMDB, Torznab, qBittorrent clients
│   └── stellarr-web/      # Leptos frontend
```

## Current Status (as of last session)
- ✅ Core architecture complete
- ✅ All domain models implemented
- ✅ TMDB, Torznab, qBittorrent integrations done
- ✅ REST API with 35+ endpoints
- ✅ Leptos UI (Dashboard, Movies, TV, Activity, Settings)
- ✅ Database schema for SQLite + PostgreSQL
- ✅ API key management (hybrid default/user keys)
- ✅ GitHub repo: https://github.com/voldigoad3421/stellarr

## What's Next (MVP remaining)
- [ ] Wire up frontend to backend (API calls)
- [ ] Test full build with `cargo build`
- [ ] Add RSS automation for new episodes
- [ ] Import/rename automation
- [ ] Notifications (Discord, etc.)

## Key Design Decisions
1. **Hybrid media model**: Base `MediaItem` for both, `Series` extends with seasons/episodes
2. **Quality scoring**: 15 quality levels with intelligent release scoring algorithm
3. **API keys**: Ship with defaults, users can override for higher rate limits
4. **Dual database**: SQLite for simple, PostgreSQL for scale

## Commands
```bash
cargo build --release          # Build
cargo run --bin stellarr serve # Run server
cargo test                     # Run tests
```

## When Continuing This Project
1. Check GitHub for any new commits/issues
2. Run `cargo check` to verify it compiles
3. Look at TODO comments in code for incomplete areas
4. The main integration point is `stellarr/src/main.rs`
