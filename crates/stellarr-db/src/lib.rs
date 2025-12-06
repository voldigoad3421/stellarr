pub mod config;
pub mod pool;
pub mod migrations;
pub mod repositories;

pub use config::{DatabaseConfig, DatabaseType};
pub use pool::DbPool;

use sqlx::{Pool, Postgres, Sqlite};

#[derive(Clone)]
pub enum Database {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}
