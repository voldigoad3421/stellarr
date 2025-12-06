-- Initial schema for SQLite
-- Media Management System Database

-- Quality Profiles Table
CREATE TABLE IF NOT EXISTS quality_profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    upgrade_allowed INTEGER NOT NULL DEFAULT 1,
    min_quality TEXT NOT NULL,
    max_quality TEXT NOT NULL,
    preferred_quality TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Media Items Table (Base table for movies and series)
CREATE TABLE IF NOT EXISTS media_items (
    id TEXT PRIMARY KEY NOT NULL,
    tmdb_id INTEGER NOT NULL UNIQUE,
    imdb_id TEXT,
    title TEXT NOT NULL,
    year INTEGER,
    overview TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    media_type TEXT NOT NULL CHECK(media_type IN ('movie', 'series')),
    status TEXT NOT NULL CHECK(status IN ('missing', 'downloading', 'available', 'failed', 'upgrade_available')),
    quality_profile_id TEXT NOT NULL REFERENCES quality_profiles(id) ON DELETE RESTRICT,
    current_quality TEXT,
    path TEXT,
    added_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    monitored INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX idx_media_items_tmdb_id ON media_items(tmdb_id);
CREATE INDEX idx_media_items_media_type ON media_items(media_type);
CREATE INDEX idx_media_items_status ON media_items(status);
CREATE INDEX idx_media_items_monitored ON media_items(monitored);

-- Series Table (TV Shows)
CREATE TABLE IF NOT EXISTS series (
    id TEXT PRIMARY KEY NOT NULL,
    tmdb_id INTEGER NOT NULL UNIQUE,
    tvdb_id INTEGER,
    imdb_id TEXT,
    title TEXT NOT NULL,
    original_title TEXT,
    year INTEGER,
    overview TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    series_status TEXT NOT NULL CHECK(series_status IN ('continuing', 'ended', 'upcoming', 'canceled')),
    quality_profile_id TEXT NOT NULL REFERENCES quality_profiles(id) ON DELETE RESTRICT,
    path TEXT,
    added_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    monitored INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX idx_series_tmdb_id ON series(tmdb_id);
CREATE INDEX idx_series_monitored ON series(monitored);

-- Seasons Table
CREATE TABLE IF NOT EXISTS seasons (
    id TEXT PRIMARY KEY NOT NULL,
    series_id TEXT NOT NULL REFERENCES series(id) ON DELETE CASCADE,
    season_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    overview TEXT,
    air_date TEXT,
    poster_path TEXT,
    episode_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL CHECK(status IN ('missing', 'partial', 'complete', 'current')),
    monitored INTEGER NOT NULL DEFAULT 1,
    UNIQUE(series_id, season_number)
);

CREATE INDEX idx_seasons_series_id ON seasons(series_id);
CREATE INDEX idx_seasons_monitored ON seasons(monitored);

-- Episodes Table
CREATE TABLE IF NOT EXISTS episodes (
    id TEXT PRIMARY KEY NOT NULL,
    season_id TEXT NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    series_id TEXT NOT NULL REFERENCES series(id) ON DELETE CASCADE,
    episode_number INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    overview TEXT,
    air_date TEXT,
    runtime INTEGER,
    still_path TEXT,
    status TEXT NOT NULL CHECK(status IN ('unaired', 'missing', 'downloading', 'downloaded', 'failed', 'upgrade_available')),
    current_quality TEXT,
    file_path TEXT,
    monitored INTEGER NOT NULL DEFAULT 1,
    added_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(season_id, episode_number)
);

CREATE INDEX idx_episodes_season_id ON episodes(season_id);
CREATE INDEX idx_episodes_series_id ON episodes(series_id);
CREATE INDEX idx_episodes_status ON episodes(status);
CREATE INDEX idx_episodes_monitored ON episodes(monitored);
CREATE INDEX idx_episodes_air_date ON episodes(air_date);

-- Indexers Table
CREATE TABLE IF NOT EXISTS indexers (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    protocol TEXT NOT NULL CHECK(protocol IN ('newznab', 'torznab')),
    url TEXT NOT NULL,
    api_key TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    priority INTEGER NOT NULL DEFAULT 50,
    categories TEXT NOT NULL,
    capabilities TEXT,
    last_used_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_indexers_enabled ON indexers(enabled);
CREATE INDEX idx_indexers_priority ON indexers(priority DESC);

-- Indexer Stats Table
CREATE TABLE IF NOT EXISTS indexer_stats (
    indexer_id TEXT PRIMARY KEY NOT NULL REFERENCES indexers(id) ON DELETE CASCADE,
    total_queries INTEGER NOT NULL DEFAULT 0,
    successful_queries INTEGER NOT NULL DEFAULT 0,
    failed_queries INTEGER NOT NULL DEFAULT 0,
    total_results INTEGER NOT NULL DEFAULT 0,
    grabbed_results INTEGER NOT NULL DEFAULT 0,
    avg_response_time_ms INTEGER,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Download Clients Table
CREATE TABLE IF NOT EXISTS download_clients (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    client_type TEXT NOT NULL CHECK(client_type IN ('qbittorrent', 'transmission', 'deluge', 'rtorrent', 'sabnzbd', 'nzbget')),
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    username TEXT,
    password TEXT,
    api_key TEXT,
    use_ssl INTEGER NOT NULL DEFAULT 0,
    url_base TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    priority INTEGER NOT NULL DEFAULT 50,
    download_directory TEXT,
    category TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_download_clients_enabled ON download_clients(enabled);
CREATE INDEX idx_download_clients_priority ON download_clients(priority DESC);

-- Downloads Table
CREATE TABLE IF NOT EXISTS downloads (
    id TEXT PRIMARY KEY NOT NULL,
    client_id TEXT NOT NULL REFERENCES download_clients(id) ON DELETE RESTRICT,
    media_item_id TEXT REFERENCES media_items(id) ON DELETE SET NULL,
    episode_id TEXT REFERENCES episodes(id) ON DELETE SET NULL,
    media_type TEXT NOT NULL CHECK(media_type IN ('movie', 'series')),
    title TEXT NOT NULL,
    download_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('queued', 'downloading', 'paused', 'completed', 'failed', 'importing', 'imported', 'removed')),
    progress REAL NOT NULL DEFAULT 0.0,
    size_bytes INTEGER,
    downloaded_bytes INTEGER,
    download_speed INTEGER,
    eta_seconds INTEGER,
    error_message TEXT,
    output_path TEXT,
    quality TEXT,
    added_at TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_downloads_client_id ON downloads(client_id);
CREATE INDEX idx_downloads_media_item_id ON downloads(media_item_id);
CREATE INDEX idx_downloads_episode_id ON downloads(episode_id);
CREATE INDEX idx_downloads_status ON downloads(status);
CREATE INDEX idx_downloads_download_id ON downloads(download_id);
