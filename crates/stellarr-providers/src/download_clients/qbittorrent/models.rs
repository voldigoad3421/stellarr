use serde::{Deserialize, Serialize};

/// Torrent information returned by qBittorrent API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    /// Torrent hash
    pub hash: String,

    /// Torrent name
    pub name: String,

    /// Total size in bytes
    pub size: u64,

    /// Progress (0.0 to 1.0)
    pub progress: f32,

    /// Download speed in bytes/sec
    pub dlspeed: u64,

    /// Upload speed in bytes/sec
    pub upspeed: u64,

    /// Torrent state
    pub state: TorrentState,

    /// Category
    #[serde(default)]
    pub category: String,

    /// Save path
    pub save_path: String,

    /// Amount downloaded in bytes
    pub downloaded: u64,

    /// Amount uploaded in bytes
    pub uploaded: u64,

    /// ETA in seconds (-1 if unknown)
    pub eta: i64,

    /// Number of seeds
    pub num_seeds: i32,

    /// Number of leechers
    pub num_leechs: i32,

    /// Ratio (uploaded/downloaded)
    pub ratio: f32,

    /// Time added (Unix timestamp)
    pub added_on: i64,

    /// Completion time (Unix timestamp, -1 if not completed)
    pub completion_on: i64,
}

/// Torrent state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TorrentState {
    /// Some error occurred, applies to paused torrents
    Error,

    /// Torrent data files is missing
    MissingFiles,

    /// Torrent is being seeded and data is being transferred
    Uploading,

    /// Torrent is paused and has finished downloading
    PausedUP,

    /// Queuing is enabled and torrent is queued for upload
    QueuedUP,

    /// Torrent is being seeded, but no connection were made
    StalledUP,

    /// Torrent has finished downloading and is being checked
    CheckingUP,

    /// Torrent is forced to uploading and ignore queue limit
    ForcedUP,

    /// Torrent is allocating disk space for download
    Allocating,

    /// Torrent is being downloaded and data is being transferred
    Downloading,

    /// Torrent has just started downloading and is fetching metadata
    MetaDL,

    /// Torrent is paused and has NOT finished downloading
    PausedDL,

    /// Queuing is enabled and torrent is queued for download
    QueuedDL,

    /// Torrent is being downloaded, but no connection were made
    StalledDL,

    /// Same as checkingUP, but torrent has NOT finished downloading
    CheckingDL,

    /// Torrent is forced to downloading to ignore queue limit
    ForcedDL,

    /// Checking resume data on qBt startup
    CheckingResumeData,

    /// Torrent is moving to another location
    Moving,

    /// Unknown status
    Unknown,
}

impl TorrentState {
    /// Returns true if the torrent is actively downloading
    pub fn is_downloading(&self) -> bool {
        matches!(
            self,
            TorrentState::Downloading
                | TorrentState::MetaDL
                | TorrentState::ForcedDL
                | TorrentState::Allocating
        )
    }

    /// Returns true if the torrent is seeding
    pub fn is_seeding(&self) -> bool {
        matches!(
            self,
            TorrentState::Uploading | TorrentState::ForcedUP | TorrentState::StalledUP
        )
    }

    /// Returns true if the torrent is paused
    pub fn is_paused(&self) -> bool {
        matches!(self, TorrentState::PausedDL | TorrentState::PausedUP)
    }

    /// Returns true if the torrent has completed downloading
    pub fn is_completed(&self) -> bool {
        matches!(
            self,
            TorrentState::Uploading
                | TorrentState::PausedUP
                | TorrentState::QueuedUP
                | TorrentState::StalledUP
                | TorrentState::CheckingUP
                | TorrentState::ForcedUP
        )
    }

    /// Returns true if the torrent has an error
    pub fn is_error(&self) -> bool {
        matches!(self, TorrentState::Error | TorrentState::MissingFiles)
    }
}

/// Response from adding a torrent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTorrentResponse {
    /// Status of the operation (e.g., "Ok.", "Fails.")
    #[serde(default)]
    pub status: String,
}

/// Login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// Login status
    #[serde(default)]
    pub status: String,
}

/// API version response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiVersion {
    /// API version string
    #[serde(default)]
    pub version: String,
}

/// Application version response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersion {
    /// Application version string
    #[serde(default)]
    pub version: String,
}

/// Torrent properties (detailed info)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentProperties {
    /// Save path
    pub save_path: String,

    /// Creation date (Unix timestamp)
    pub creation_date: i64,

    /// Piece size in bytes
    pub piece_size: u64,

    /// Comment
    #[serde(default)]
    pub comment: String,

    /// Total wasted in bytes
    pub total_wasted: u64,

    /// Total uploaded in bytes
    pub total_uploaded: u64,

    /// Total uploaded for this session in bytes
    pub total_uploaded_session: u64,

    /// Total downloaded in bytes
    pub total_downloaded: u64,

    /// Total downloaded for this session in bytes
    pub total_downloaded_session: u64,

    /// Upload limit in bytes/sec (-1 if unlimited)
    pub up_limit: i64,

    /// Download limit in bytes/sec (-1 if unlimited)
    pub dl_limit: i64,

    /// Time elapsed in seconds
    pub time_elapsed: i64,

    /// Seeding time in seconds
    pub seeding_time: i64,

    /// Number of connections
    pub nb_connections: i32,

    /// Connection limit (-1 if unlimited)
    pub nb_connections_limit: i32,

    /// Share ratio
    pub share_ratio: f32,

    /// Addition date (Unix timestamp)
    pub addition_date: i64,

    /// Completion date (Unix timestamp, -1 if not completed)
    pub completion_date: i64,

    /// Created by
    #[serde(default)]
    pub created_by: String,

    /// Download speed average in bytes/sec
    pub dl_speed_avg: u64,

    /// Download speed in bytes/sec
    pub dl_speed: u64,

    /// ETA in seconds
    pub eta: i64,

    /// Last seen complete (Unix timestamp, -1 if never)
    pub last_seen: i64,

    /// Number of peers
    pub peers: i32,

    /// Total number of peers
    pub peers_total: i32,

    /// Number of pieces that have been downloaded
    pub pieces_have: i32,

    /// Total number of pieces
    pub pieces_num: i32,

    /// Number of seeds
    pub seeds: i32,

    /// Total number of seeds
    pub seeds_total: i32,

    /// Upload speed average in bytes/sec
    pub up_speed_avg: u64,

    /// Upload speed in bytes/sec
    pub up_speed: u64,
}
