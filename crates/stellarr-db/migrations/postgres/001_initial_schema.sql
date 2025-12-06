-- Initial schema for PostgreSQL
-- Media Management System Database

-- Create ENUM types
CREATE TYPE media_type AS ENUM ('movie', 'series');
CREATE TYPE media_status AS ENUM ('missing', 'downloading', 'available', 'failed', 'upgrade_available');
CREATE TYPE series_status AS ENUM ('continuing', 'ended', 'upcoming', 'canceled');
CREATE TYPE season_status AS ENUM ('missing', 'partial', 'complete', 'current');
CREATE TYPE episode_status AS ENUM ('unaired', 'missing', 'downloading', 'downloaded', 'failed', 'upgrade_available');
CREATE TYPE indexer_protocol AS ENUM ('newznab', 'torznab');
CREATE TYPE download_client_type AS ENUM ('qbittorrent', 'transmission', 'deluge', 'rtorrent', 'sabnzbd', 'nzbget');
CREATE TYPE download_status AS ENUM ('queued', 'downloading', 'paused', 'completed', 'failed', 'importing', 'imported', 'removed');
CREATE TYPE quality AS ENUM ('unknown', 'sd240', 'sd480', 'sd576', 'webdl720', 'hdtv720', 'bluray720', 'webdl1080', 'hdtv1080', 'bluray1080', 'webdl2160', 'hdtv2160', 'bluray2160', 'remux1080', 'remux2160');

-- Quality Profiles Table
CREATE TABLE quality_profiles (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    upgrade_allowed BOOLEAN NOT NULL DEFAULT TRUE,
    min_quality quality NOT NULL,
    max_quality quality NOT NULL,
    preferred_quality quality NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Media Items Table
CREATE TABLE media_items (
    id UUID PRIMARY KEY,
    tmdb_id BIGINT NOT NULL UNIQUE,
    imdb_id VARCHAR(20),
    title VARCHAR(500) NOT NULL,
    year INTEGER,
    overview TEXT,
    poster_path VARCHAR(500),
    backdrop_path VARCHAR(500),
    media_type media_type NOT NULL,
    status media_status NOT NULL,
    quality_profile_id UUID NOT NULL REFERENCES quality_profiles(id) ON DELETE RESTRICT,
    current_quality VARCHAR(50),
    path TEXT,
    added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    monitored BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_media_items_tmdb_id ON media_items(tmdb_id);
CREATE INDEX idx_media_items_media_type ON media_items(media_type);
CREATE INDEX idx_media_items_status ON media_items(status);
CREATE INDEX idx_media_items_monitored ON media_items(monitored);

-- Series Table
CREATE TABLE series (
    id UUID PRIMARY KEY,
    tmdb_id BIGINT NOT NULL UNIQUE,
    tvdb_id BIGINT,
    imdb_id VARCHAR(20),
    title VARCHAR(500) NOT NULL,
    original_title VARCHAR(500),
    year INTEGER,
    overview TEXT,
    poster_path VARCHAR(500),
    backdrop_path VARCHAR(500),
    series_status series_status NOT NULL,
    quality_profile_id UUID NOT NULL REFERENCES quality_profiles(id) ON DELETE RESTRICT,
    path TEXT,
    added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    monitored BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_series_tmdb_id ON series(tmdb_id);
CREATE INDEX idx_series_monitored ON series(monitored);

-- Seasons Table
CREATE TABLE seasons (
    id UUID PRIMARY KEY,
    series_id UUID NOT NULL REFERENCES series(id) ON DELETE CASCADE,
    season_number INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    overview TEXT,
    air_date DATE,
    poster_path VARCHAR(500),
    episode_count INTEGER NOT NULL DEFAULT 0,
    status season_status NOT NULL,
    monitored BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE(series_id, season_number)
);

CREATE INDEX idx_seasons_series_id ON seasons(series_id);

-- Episodes Table
CREATE TABLE episodes (
    id UUID PRIMARY KEY,
    season_id UUID NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    series_id UUID NOT NULL REFERENCES series(id) ON DELETE CASCADE,
    episode_number INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    title VARCHAR(500) NOT NULL,
    overview TEXT,
    air_date DATE,
    runtime INTEGER,
    still_path VARCHAR(500),
    status episode_status NOT NULL,
    current_quality VARCHAR(50),
    file_path TEXT,
    monitored BOOLEAN NOT NULL DEFAULT TRUE,
    added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(season_id, episode_number)
);

CREATE INDEX idx_episodes_series_id ON episodes(series_id);
CREATE INDEX idx_episodes_status ON episodes(status);

-- Indexers Table
CREATE TABLE indexers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    protocol indexer_protocol NOT NULL,
    url VARCHAR(500) NOT NULL,
    api_key VARCHAR(255),
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    priority INTEGER NOT NULL DEFAULT 50,
    categories JSONB NOT NULL,
    capabilities JSONB,
    last_used_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_indexers_enabled ON indexers(enabled);

-- Download Clients Table
CREATE TABLE download_clients (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    client_type download_client_type NOT NULL,
    host VARCHAR(255) NOT NULL,
    port INTEGER NOT NULL,
    username VARCHAR(255),
    password VARCHAR(255),
    api_key VARCHAR(255),
    use_ssl BOOLEAN NOT NULL DEFAULT FALSE,
    url_base VARCHAR(255),
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    priority INTEGER NOT NULL DEFAULT 50,
    download_directory TEXT,
    category VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_download_clients_enabled ON download_clients(enabled);

-- Downloads Table
CREATE TABLE downloads (
    id UUID PRIMARY KEY,
    client_id UUID NOT NULL REFERENCES download_clients(id) ON DELETE RESTRICT,
    media_item_id UUID REFERENCES media_items(id) ON DELETE SET NULL,
    episode_id UUID REFERENCES episodes(id) ON DELETE SET NULL,
    media_type media_type NOT NULL,
    title VARCHAR(500) NOT NULL,
    download_id VARCHAR(255) NOT NULL,
    status download_status NOT NULL,
    progress DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    size_bytes BIGINT,
    downloaded_bytes BIGINT,
    download_speed BIGINT,
    eta_seconds BIGINT,
    error_message TEXT,
    output_path TEXT,
    quality VARCHAR(50),
    added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_downloads_status ON downloads(status);
