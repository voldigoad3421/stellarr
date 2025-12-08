//! Database migrations module
//!
//! This module provides migration management for both SQLite and PostgreSQL.
//! Migrations are embedded into the binary using sqlx::migrate! macro.
//!
//! Run migrations using the Database::migrate() method in pool.rs
//! which uses sqlx::migrate! macro to embed migrations at compile time.
//!
//! Migrations are located in:
//! - ./migrations/sqlite/ for SQLite
//! - ./migrations/postgres/ for PostgreSQL
