use crate::{config::{DatabaseConfig, DatabaseType}, Database};
use sqlx::{Pool, Postgres, Sqlite, SqlitePool, PgPool};
use std::time::Duration;
use tracing::{info, debug};

/// Type alias for database pool
pub type DbPool = Database;

impl Database {
    /// Connect to a database using the provided configuration
    pub async fn connect(config: &DatabaseConfig) -> anyhow::Result<Self> {
        let db_type = config.get_database_type()?;
        
        info!("Connecting to {:?} database: {}", db_type, config.database_url);
        
        match db_type {
            DatabaseType::Postgres => {
                let pool = create_postgres_pool(config).await?;
                Ok(Database::Postgres(pool))
            }
            DatabaseType::Sqlite => {
                let pool = create_sqlite_pool(config).await?;
                Ok(Database::Sqlite(pool))
            }
        }
    }

    /// Run database migrations
    pub async fn migrate(&self) -> anyhow::Result<()> {
        info!("Running database migrations");
        
        match self {
            Database::Postgres(pool) => {
                sqlx::migrate!("./migrations/postgres")
                    .run(pool)
                    .await?;
            }
            Database::Sqlite(pool) => {
                sqlx::migrate!("./migrations/sqlite")
                    .run(pool)
                    .await?;
            }
        }
        
        info!("Database migrations completed successfully");
        Ok(())
    }

    /// Close the database connection pool
    pub async fn close(&self) {
        match self {
            Database::Postgres(pool) => pool.close().await,
            Database::Sqlite(pool) => pool.close().await,
        }
    }

    /// Get the database type
    pub fn database_type(&self) -> DatabaseType {
        match self {
            Database::Postgres(_) => DatabaseType::Postgres,
            Database::Sqlite(_) => DatabaseType::Sqlite,
        }
    }

    /// Check if the connection is healthy
    pub async fn ping(&self) -> anyhow::Result<()> {
        match self {
            Database::Postgres(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
            Database::Sqlite(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
        }
        Ok(())
    }
}

/// Create a PostgreSQL connection pool
async fn create_postgres_pool(config: &DatabaseConfig) -> anyhow::Result<Pool<Postgres>> {
    use sqlx::postgres::PgPoolOptions;

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout_seconds))
        .idle_timeout(if config.idle_timeout_seconds > 0 {
            Some(Duration::from_secs(config.idle_timeout_seconds))
        } else {
            None
        })
        .max_lifetime(if config.max_lifetime_seconds > 0 {
            Some(Duration::from_secs(config.max_lifetime_seconds))
        } else {
            None
        })
        .connect(&config.database_url)
        .await?;

    debug!("PostgreSQL connection pool created with {} max connections", config.max_connections);
    
    Ok(pool)
}

/// Create a SQLite connection pool
async fn create_sqlite_pool(config: &DatabaseConfig) -> anyhow::Result<Pool<Sqlite>> {
    use sqlx::sqlite::SqlitePoolOptions;

    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout_seconds))
        .idle_timeout(if config.idle_timeout_seconds > 0 {
            Some(Duration::from_secs(config.idle_timeout_seconds))
        } else {
            None
        })
        .max_lifetime(if config.max_lifetime_seconds > 0 {
            Some(Duration::from_secs(config.max_lifetime_seconds))
        } else {
            None
        })
        .connect(&config.database_url)
        .await?;

    // Enable foreign keys for SQLite
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;

    // Enable WAL mode for better concurrency
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await?;

    debug!("SQLite connection pool created with {} max connections", config.max_connections);
    
    Ok(pool)
}
