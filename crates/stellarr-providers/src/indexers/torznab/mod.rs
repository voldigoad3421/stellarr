//! Torznab/Newznab protocol implementation for indexer communication
//!
//! This module provides a complete implementation of the Torznab/Newznab protocol,
//! which is the standard API used by indexers like Jackett and Prowlarr.

pub mod client;
pub mod error;
pub mod models;

pub use client::TorznabClient;
pub use error::{Result, TorznabError};
pub use models::{
    Capabilities, Category, Item, SearchResponse, TorznabAttr, TorznabResponse,
};
