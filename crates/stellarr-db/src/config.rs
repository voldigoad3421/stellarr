use serde::{Deserialize, Serialize};

/// Database backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    /// SQLite database (file-based, suitable for single-user)
    Sqlite,
    /// PostgreSQL database (server-based, suitable for multi-user)
    Postgres,
}

impl DatabaseType {
    /// Auto-detect database type from connection URL
    pub fn from_url(url: &str) -> Option<Self> {
        if url.starts_with("sqlite:") || url.starts_with("sqlite://") {
            Some(DatabaseType::Sqlite)
        } else if url.starts_with("postgres:") || url.starts_with("postgresql:") {
            Some(DatabaseType::Postgres)
        } else {
            None
        }
    }

    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseType::Sqlite => "sqlite",
            DatabaseType::Postgres => "postgres",
        }
    }
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database backend type (auto-detected from URL if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_type: Option<DatabaseType>,

    /// Database connection URL
    /// SQLite: sqlite://path/to/database.db or sqlite::memory:
    /// PostgreSQL: postgresql://user:pass@host:port/dbname
    pub database_url: String,

    /// Maximum number of connections in the pool
    pub max_connections: u32,

    /// Minimum number of idle connections to maintain
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// Connection timeout in seconds
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_seconds: u64,

    /// Idle connection timeout in seconds (0 = no timeout)
    #[serde(default)]
    pub idle_timeout_seconds: u64,

    /// Maximum lifetime of a connection in seconds (0 = no limit)
    #[serde(default)]
    pub max_lifetime_seconds: u64,

    /// Whether to run migrations automatically on startup
    #[serde(default = "default_true")]
    pub auto_migrate: bool,
}

fn default_min_connections() -> u32 {
    1
}

fn default_connect_timeout() -> u64 {
    30
}

fn default_true() -> bool {
    true
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_type: Some(DatabaseType::Sqlite),
            database_url: "sqlite://stellarr.db".to_string(),
            max_connections: 5,
            min_connections: 1,
            connect_timeout_seconds: 30,
            idle_timeout_seconds: 600, // 10 minutes
            max_lifetime_seconds: 1800, // 30 minutes
            auto_migrate: true,
        }
    }
}

impl DatabaseConfig {
    /// Get the database type, auto-detecting from URL if not explicitly set
    pub fn get_database_type(&self) -> anyhow::Result<DatabaseType> {
        if let Some(db_type) = self.database_type {
            return Ok(db_type);
        }

        DatabaseType::from_url(&self.database_url)
            .ok_or_else(|| anyhow::anyhow!("Could not determine database type from URL: {}", self.database_url))
    }

    /// Create a configuration for SQLite
    pub fn sqlite(path: impl Into<String>) -> Self {
        Self {
            database_type: Some(DatabaseType::Sqlite),
            database_url: format!("sqlite://{}", path.into()),
            max_connections: 5,
            min_connections: 1,
            connect_timeout_seconds: 30,
            idle_timeout_seconds: 600,
            max_lifetime_seconds: 1800,
            auto_migrate: true,
        }
    }

    /// Create a configuration for PostgreSQL
    pub fn postgres(url: impl Into<String>) -> Self {
        Self {
            database_type: Some(DatabaseType::Postgres),
            database_url: url.into(),
            max_connections: 20,
            min_connections: 2,
            connect_timeout_seconds: 30,
            idle_timeout_seconds: 600,
            max_lifetime_seconds: 1800,
            auto_migrate: true,
        }
    }

    /// Create an in-memory SQLite configuration (useful for testing)
    pub fn sqlite_memory() -> Self {
        Self {
            database_type: Some(DatabaseType::Sqlite),
            database_url: "sqlite::memory:".to_string(),
            max_connections: 1,
            min_connections: 1,
            connect_timeout_seconds: 30,
            idle_timeout_seconds: 0,
            max_lifetime_seconds: 0,
            auto_migrate: true,
        }
    }
}
