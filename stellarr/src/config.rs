use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use stellarr_db::DatabaseConfig;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub api_keys: ApiKeysConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// API Keys configuration with built-in defaults
///
/// All keys are optional - Stellarr ships with defaults that work out of the box.
/// Users can override with their own keys for higher rate limits or to support services.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeysConfig {
    /// TMDB (The Movie Database) - for movie/TV metadata
    #[serde(default = "ApiKeysConfig::default_tmdb")]
    pub tmdb_access_token: String,

    /// Have I Been Pwned - for checking compromised credentials
    #[serde(default)]
    pub hibp_api_key: Option<String>,

    /// Fanart.tv - for additional artwork
    #[serde(default = "ApiKeysConfig::default_fanart")]
    pub fanart_api_key: String,

    /// OMDb - for Rotten Tomatoes scores
    #[serde(default)]
    pub omdb_api_key: Option<String>,

    /// Trakt.tv - for watchlist sync
    #[serde(default)]
    pub trakt_client_id: Option<String>,
    #[serde(default)]
    pub trakt_client_secret: Option<String>,
}

impl ApiKeysConfig {
    // =========================================================
    // DEFAULT API KEYS
    // =========================================================
    // These are the project's shared keys. Users get these automatically.
    // Replace "YOUR_PROJECT_..._HERE" with actual keys before release.
    //
    // To get your own keys:
    // - TMDB: https://themoviedb.org/settings/api (free)
    // - Fanart: https://fanart.tv/get-an-api-key/ (free)
    // =========================================================

    fn default_tmdb() -> String {
        // TODO: Add your project's TMDB API key here before release
        "YOUR_PROJECT_TMDB_KEY_HERE".to_string()
    }

    fn default_fanart() -> String {
        // TODO: Add your project's Fanart.tv key here before release
        "YOUR_PROJECT_FANART_KEY_HERE".to_string()
    }

    /// Get TMDB token, preferring user-provided over default
    pub fn tmdb(&self) -> &str {
        &self.tmdb_access_token
    }

    /// Get Fanart.tv key
    pub fn fanart(&self) -> &str {
        &self.fanart_api_key
    }

    /// Check if HIBP is configured (no default, user must provide)
    pub fn hibp(&self) -> Option<&str> {
        self.hibp_api_key.as_deref()
    }

    /// Check if OMDb is configured
    pub fn omdb(&self) -> Option<&str> {
        self.omdb_api_key.as_deref()
    }

    /// Check if Trakt is fully configured
    pub fn trakt(&self) -> Option<(&str, &str)> {
        match (&self.trakt_client_id, &self.trakt_client_secret) {
            (Some(id), Some(secret)) => Some((id.as_str(), secret.as_str())),
            _ => None,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Server defaults
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            // Database defaults
            .set_default("database.database_type", "sqlite")?
            .set_default("database.database_url", "sqlite:///app/data/stellarr.db?mode=rwc")?
            .set_default("database.max_connections", 5)?
            .set_default("api_keys.tmdb_access_token", "YOUR_PROJECT_TMDB_KEY_HERE")?
            .set_default("api_keys.fanart_api_key", "YOUR_PROJECT_FANART_KEY_HERE")?
            // API key defaults are handled by ApiKeysConfig::default_*
            // Load from config file if exists
            .add_source(File::with_name("config").required(false))
            // Override with environment variables
            // e.g., STELLARR_API_KEYS_TMDB_ACCESS_TOKEN=xxx
            .add_source(Environment::with_prefix("STELLARR").separator("_"))
            .build()?;

        config.try_deserialize()
    }
}
