//! Release search results and scoring
//!
//! When searching indexers, we get back releases - potential downloads
//! that match our search criteria. This module defines:
//! - Release structure from indexer results
//! - Scoring algorithm to rank releases
//! - Filtering and selection logic
//!
//! The scoring algorithm is critical for automatically selecting the
//! best release when multiple options are available.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::indexer::IndexerProtocol;
use super::quality::{Quality, QualityProfile};

/// Release from an indexer search
///
/// Represents a single search result from an indexer.
/// This is the raw data we get back before any processing or scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    /// Internal ID (generated when we create this)
    pub id: Uuid,

    /// Indexer that provided this release
    pub indexer_id: Uuid,

    /// Indexer name (denormalized for display)
    pub indexer_name: String,

    /// Protocol used (torrent or usenet)
    pub protocol: IndexerProtocol,

    /// Release title/name
    pub title: String,

    /// Download URL or magnet link
    pub download_url: String,

    /// InfoHash (for torrents)
    pub info_hash: Option<String>,

    /// Size in bytes
    pub size_bytes: i64,

    /// Number of seeders (for torrents)
    pub seeders: Option<i32>,

    /// Number of leechers (for torrents)
    pub leechers: Option<i32>,

    /// Parsed quality from the title
    pub quality: Quality,

    /// Release group/team
    pub group: Option<String>,

    /// Publication/upload date
    pub published_at: DateTime<Utc>,

    /// When we found this release
    pub discovered_at: DateTime<Utc>,

    /// Calculated score (higher = better)
    /// This is calculated when scoring against a quality profile
    pub score: Option<i32>,
}

impl Release {
    /// Create a new release from indexer data
    pub fn new(
        indexer_id: Uuid,
        indexer_name: String,
        protocol: IndexerProtocol,
        title: String,
        download_url: String,
        size_bytes: i64,
    ) -> Self {
        let quality = Quality::parse_from_release_name(&title);
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            indexer_id,
            indexer_name,
            protocol,
            title: title.clone(),
            download_url,
            info_hash: None,
            size_bytes,
            seeders: None,
            leechers: None,
            quality,
            group: parse_group_from_title(&title),
            published_at: now,
            discovered_at: now,
            score: None,
        }
    }

    /// Calculate and set the score for this release
    ///
    /// The scoring algorithm considers:
    /// 1. Quality match (primary factor)
    /// 2. Seeder count (for torrents)
    /// 3. Release group reputation
    /// 4. Size reasonableness
    /// 5. Age (prefer recent releases)
    pub fn calculate_score(&mut self, profile: &QualityProfile, preferred_groups: &[String]) {
        let mut score = 0;

        // Quality scoring (0-10000 points)
        score += profile.score_quality(&self.quality);

        // Seeder bonus (for torrents, 0-500 points)
        if let Some(seeders) = self.seeders {
            score += (seeders.min(50) * 10) as i32; // Max 500 points
        }

        // Preferred group bonus (0-1000 points)
        if let Some(ref group) = self.group {
            if preferred_groups
                .iter()
                .any(|g| g.eq_ignore_ascii_case(group))
            {
                score += 1000;
            }
        }

        // Known quality groups bonus (0-200 points)
        if let Some(ref group) = self.group {
            if is_trusted_group(group) {
                score += 200;
            }
        }

        // Size scoring (penalty for unreasonable sizes)
        score += self.score_size();

        // Age penalty (prefer recent releases, max -100 points)
        let age_days = (Utc::now() - self.published_at).num_days();
        if age_days > 30 {
            score -= ((age_days - 30).min(100)) as i32;
        }

        // Protocol bonus (slight preference for usenet over torrents)
        if matches!(self.protocol, IndexerProtocol::Newznab) {
            score += 50;
        }

        self.score = Some(score);
    }

    /// Score the size reasonableness
    ///
    /// Too small = probably low quality or fake
    /// Too large = probably uncompressed or includes extras
    fn score_size(&self) -> i32 {
        let size_gb = self.size_bytes as f64 / 1_073_741_824.0;

        match self.quality.resolution() {
            super::quality::Resolution::Sd480 => {
                // SD: expect 0.5-2 GB
                if size_gb < 0.3 {
                    -200
                } else if size_gb > 4.0 {
                    -100
                } else {
                    0
                }
            }
            super::quality::Resolution::Hd720 => {
                // 720p: expect 2-6 GB
                if size_gb < 0.8 {
                    -200
                } else if size_gb > 10.0 {
                    -100
                } else {
                    0
                }
            }
            super::quality::Resolution::Hd1080 => {
                // 1080p: expect 4-15 GB
                if size_gb < 2.0 {
                    -200
                } else if size_gb > 25.0 {
                    -100
                } else {
                    0
                }
            }
            super::quality::Resolution::Uhd2160 => {
                // 4K: expect 15-60 GB
                if size_gb < 10.0 {
                    -200
                } else if size_gb > 100.0 {
                    -100
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// Get human-readable size
    pub fn size_string(&self) -> String {
        format_bytes(self.size_bytes)
    }

    /// Check if this release is acceptable for a quality profile
    pub fn is_acceptable(&self, profile: &QualityProfile) -> bool {
        profile.is_acceptable(&self.quality)
    }

    /// Check if this release should be preferred (auto-grab)
    pub fn is_preferred(&self, profile: &QualityProfile) -> bool {
        profile.is_preferred(&self.quality)
    }
}

/// Parse the release group from a title
///
/// Common format: "Title.Year.Quality.Codec-GROUP"
/// or "Title S01E01 Quality-GROUP"
fn parse_group_from_title(title: &str) -> Option<String> {
    // Look for the last dash followed by word characters
    title
        .rsplit_once('-')
        .and_then(|(_, group)| {
            let group = group.trim();
            // Remove common extensions
            let group = group
                .trim_end_matches(".mkv")
                .trim_end_matches(".mp4")
                .trim_end_matches(".avi");

            // Only return if it looks like a valid group name (alphanumeric, not too long)
            if group.len() <= 30 && group.chars().any(|c| c.is_alphanumeric()) {
                Some(group.to_string())
            } else {
                None
            }
        })
}

/// Check if a group is a known trusted/quality group
///
/// This is a curated list of scene groups known for quality releases.
/// In production, this might be user-configurable.
fn is_trusted_group(group: &str) -> bool {
    const TRUSTED_GROUPS: &[&str] = &[
        // Movie groups
        "SPARKS",
        "EVO",
        "FGT",
        "RARBG",
        "YTS",
        "YIFY",
        "BLOW",
        "DEFLATE",
        "GECKOS",
        "ROVERS",
        // TV groups
        "FLEET",
        "CAKES",
        "SVA",
        "MKVTV",
        "NTb",
        "MIXED",
        "TOMMY",
        "GOSSIP",
        "INFLATE",
    ];

    TRUSTED_GROUPS
        .iter()
        .any(|&g| g.eq_ignore_ascii_case(group))
}

/// Format bytes into human-readable size
fn format_bytes(bytes: i64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Release comparison and selection utilities
pub struct ReleaseSelector;

impl ReleaseSelector {
    /// Select the best release from a list
    ///
    /// Returns the highest-scoring release that meets the criteria.
    pub fn select_best(mut releases: Vec<Release>, profile: &QualityProfile) -> Option<Release> {
        // Filter to only acceptable releases
        releases.retain(|r| r.is_acceptable(profile));

        if releases.is_empty() {
            return None;
        }

        // Sort by score (highest first)
        releases.sort_by(|a, b| {
            b.score
                .unwrap_or(0)
                .cmp(&a.score.unwrap_or(0))
        });

        releases.into_iter().next()
    }

    /// Filter releases by minimum seeders (for torrents)
    pub fn filter_by_seeders(releases: Vec<Release>, min_seeders: i32) -> Vec<Release> {
        releases
            .into_iter()
            .filter(|r| {
                // Allow usenet releases through
                if matches!(r.protocol, IndexerProtocol::Newznab) {
                    return true;
                }

                // For torrents, check seeders
                r.seeders.unwrap_or(0) >= min_seeders
            })
            .collect()
    }

    /// Filter releases by preferred groups
    pub fn filter_by_groups(releases: Vec<Release>, groups: &[String]) -> Vec<Release> {
        if groups.is_empty() {
            return releases;
        }

        releases
            .into_iter()
            .filter(|r| {
                r.group
                    .as_ref()
                    .map(|g| groups.iter().any(|pg| pg.eq_ignore_ascii_case(g)))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Filter releases by size range
    pub fn filter_by_size(
        releases: Vec<Release>,
        min_bytes: Option<i64>,
        max_bytes: Option<i64>,
    ) -> Vec<Release> {
        releases
            .into_iter()
            .filter(|r| {
                if let Some(min) = min_bytes {
                    if r.size_bytes < min {
                        return false;
                    }
                }
                if let Some(max) = max_bytes {
                    if r.size_bytes > max {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_group() {
        assert_eq!(
            parse_group_from_title("Movie.2023.1080p.BluRay.x264-SPARKS"),
            Some("SPARKS".to_string())
        );
        assert_eq!(
            parse_group_from_title("Show.S01E01.720p.WEB-DL-FGT"),
            Some("FGT".to_string())
        );
        assert_eq!(
            parse_group_from_title("Movie.2023.1080p.BluRay.x264-SPARKS.mkv"),
            Some("SPARKS".to_string())
        );
    }

    #[test]
    fn test_trusted_group() {
        assert!(is_trusted_group("SPARKS"));
        assert!(is_trusted_group("sparks")); // Case insensitive
        assert!(!is_trusted_group("UNKNOWN"));
    }

    #[test]
    fn test_release_scoring() {
        let mut release = Release::new(
            Uuid::new_v4(),
            "Test Indexer".to_string(),
            IndexerProtocol::Torznab,
            "Movie.2023.1080p.BluRay.x264-SPARKS".to_string(),
            "http://example.com/download".to_string(),
            8_589_934_592, // 8 GB
        );

        release.seeders = Some(100);

        let profile = QualityProfile::hd_1080p();
        release.calculate_score(&profile, &[]);

        assert!(release.score.is_some());
        assert!(release.score.unwrap() > 0);
    }

    #[test]
    fn test_release_selector() {
        let profile = QualityProfile::hd();

        let mut releases = vec![
            Release::new(
                Uuid::new_v4(),
                "Indexer".to_string(),
                IndexerProtocol::Torznab,
                "Movie.2023.720p.WEB-DL-FGT".to_string(),
                "url1".to_string(),
                4_294_967_296,
            ),
            Release::new(
                Uuid::new_v4(),
                "Indexer".to_string(),
                IndexerProtocol::Torznab,
                "Movie.2023.1080p.BluRay-SPARKS".to_string(),
                "url2".to_string(),
                8_589_934_592,
            ),
        ];

        // Score all releases
        for release in &mut releases {
            release.calculate_score(&profile, &[]);
        }

        let best = ReleaseSelector::select_best(releases, &profile);
        assert!(best.is_some());

        let best = best.unwrap();
        assert_eq!(best.quality, Quality::Bluray1080);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1_048_576), "1.00 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.00 GB");
    }
}
