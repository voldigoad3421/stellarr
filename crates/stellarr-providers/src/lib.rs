//! External service integrations for Stellarr
//!
//! This crate provides integrations for:
//! - TMDB (The Movie Database) - metadata provider
//! - Indexers (Jackett, Prowlarr) - torrent/usenet search
//! - Download clients (qBittorrent, Transmission, SABnzbd) - download management

pub mod tmdb;
pub mod indexers;
pub mod download_clients;

pub use tmdb::TmdbClient;
