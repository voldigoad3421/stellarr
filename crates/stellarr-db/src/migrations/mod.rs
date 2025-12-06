//! Database migrations module
//!
//! This module provides migration management for both SQLite and PostgreSQL.
//! Migrations are embedded into the binary using sqlx::migrate! macro.

/// Run migrations for the database
///
/// This is handled by the Database::migrate() method in pool.rs
/// using sqlx::migrate! macro which embeds migrations at compile time.
///
/// Migrations are located in:
/// - ./migrations/sqlite/ for SQLite
/// - ./migrations/postgres/ for PostgreSQL
