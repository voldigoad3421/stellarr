//! Quality profile and quality tier system
//!
//! This module defines the quality hierarchy used for:
//! - Setting target quality for downloads
//! - Determining if an upgrade is warranted
//! - Scoring and ranking release candidates
//!
//! The quality system is intentionally comprehensive, covering standard
//! definitions used across usenet and torrent indexers.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Media quality tiers
///
/// Ordered from lowest to highest quality. This ordering is critical for
/// upgrade logic - we can derive `PartialOrd` and `Ord` to compare qualities.
///
/// Source types (WEB-DL, BluRay, etc.) are embedded in the quality tier
/// to provide a more precise ranking that matches real-world preferences.
///
/// Design decision: We use explicit quality tiers rather than a generic
/// "quality score" to maintain human readability and debuggability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "quality", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Quality {
    /// Unknown or unrecognized quality
    Unknown,

    /// 240p (extremely rare, mostly for compatibility)
    Sd240,

    /// 480p (DVD quality)
    Sd480,

    /// 576p (PAL DVD quality)
    Sd576,

    /// 720p WEB-DL (streaming rip)
    WebDl720,

    /// 720p HDTV broadcast
    Hdtv720,

    /// 720p BluRay
    Bluray720,

    /// 1080p WEB-DL (streaming rip)
    WebDl1080,

    /// 1080p HDTV broadcast
    Hdtv1080,

    /// 1080p BluRay
    Bluray1080,

    /// 2160p (4K) WEB-DL
    WebDl2160,

    /// 2160p (4K) HDTV broadcast
    Hdtv2160,

    /// 2160p (4K) BluRay
    Bluray2160,

    /// 1080p Remux (uncompressed BluRay rip)
    Remux1080,

    /// 2160p (4K) Remux (uncompressed 4K BluRay rip)
    Remux2160,
}

impl Quality {
    /// Get the resolution category for this quality
    pub fn resolution(&self) -> Resolution {
        match self {
            Quality::Unknown => Resolution::Unknown,
            Quality::Sd240 => Resolution::Sd240,
            Quality::Sd480 | Quality::Sd576 => Resolution::Sd480,
            Quality::WebDl720 | Quality::Hdtv720 | Quality::Bluray720 => Resolution::Hd720,
            Quality::WebDl1080
            | Quality::Hdtv1080
            | Quality::Bluray1080
            | Quality::Remux1080 => Resolution::Hd1080,
            Quality::WebDl2160
            | Quality::Hdtv2160
            | Quality::Bluray2160
            | Quality::Remux2160 => Resolution::Uhd2160,
        }
    }

    /// Get a human-readable display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Quality::Unknown => "Unknown",
            Quality::Sd240 => "240p",
            Quality::Sd480 => "480p",
            Quality::Sd576 => "576p",
            Quality::WebDl720 => "720p WEB-DL",
            Quality::Hdtv720 => "720p HDTV",
            Quality::Bluray720 => "720p BluRay",
            Quality::WebDl1080 => "1080p WEB-DL",
            Quality::Hdtv1080 => "1080p HDTV",
            Quality::Bluray1080 => "1080p BluRay",
            Quality::WebDl2160 => "2160p WEB-DL",
            Quality::Hdtv2160 => "2160p HDTV",
            Quality::Bluray2160 => "2160p BluRay",
            Quality::Remux1080 => "1080p Remux",
            Quality::Remux2160 => "2160p Remux",
        }
    }

    /// Parse a quality from a release name or quality string
    ///
    /// This is a best-effort parser that looks for common quality indicators.
    /// Returns `Unknown` if no clear quality can be determined.
    pub fn parse_from_release_name(name: &str) -> Self {
        let name_upper = name.to_uppercase();

        // Check for remux first (most specific)
        if name_upper.contains("REMUX") {
            if name_upper.contains("2160P") || name_upper.contains("4K") {
                return Quality::Remux2160;
            } else if name_upper.contains("1080P") {
                return Quality::Remux1080;
            }
        }

        // Check for BluRay
        if name_upper.contains("BLURAY") || name_upper.contains("BLU-RAY") {
            if name_upper.contains("2160P") || name_upper.contains("4K") {
                return Quality::Bluray2160;
            } else if name_upper.contains("1080P") {
                return Quality::Bluray1080;
            } else if name_upper.contains("720P") {
                return Quality::Bluray720;
            }
        }

        // Check for WEB-DL / WEBRip
        if name_upper.contains("WEB-DL")
            || name_upper.contains("WEBDL")
            || name_upper.contains("WEBRIP")
        {
            if name_upper.contains("2160P") || name_upper.contains("4K") {
                return Quality::WebDl2160;
            } else if name_upper.contains("1080P") {
                return Quality::WebDl1080;
            } else if name_upper.contains("720P") {
                return Quality::WebDl720;
            }
        }

        // Check for HDTV
        if name_upper.contains("HDTV") {
            if name_upper.contains("2160P") || name_upper.contains("4K") {
                return Quality::Hdtv2160;
            } else if name_upper.contains("1080P") {
                return Quality::Hdtv1080;
            } else if name_upper.contains("720P") {
                return Quality::Hdtv720;
            }
        }

        // Generic resolution checks (fallback)
        if name_upper.contains("2160P") || name_upper.contains("4K") {
            return Quality::WebDl2160; // Default to web-dl for 4K
        } else if name_upper.contains("1080P") {
            return Quality::WebDl1080;
        } else if name_upper.contains("720P") {
            return Quality::WebDl720;
        } else if name_upper.contains("480P") {
            return Quality::Sd480;
        }

        Quality::Unknown
    }

    /// Check if this quality is better than another
    pub fn is_better_than(&self, other: &Quality) -> bool {
        self > other
    }
}

/// Resolution categories
///
/// Simplified resolution groupings for UI display and filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Resolution {
    Unknown,
    Sd240,
    Sd480,
    Hd720,
    Hd1080,
    Uhd2160,
}

impl Resolution {
    /// Get a display name for this resolution
    pub fn display_name(&self) -> &'static str {
        match self {
            Resolution::Unknown => "Unknown",
            Resolution::Sd240 => "240p",
            Resolution::Sd480 => "480p/576p",
            Resolution::Hd720 => "720p",
            Resolution::Hd1080 => "1080p",
            Resolution::Uhd2160 => "2160p/4K",
        }
    }
}

/// Quality profile defining acceptable quality range for media
///
/// A quality profile specifies:
/// - Minimum acceptable quality (won't download below this)
/// - Maximum desired quality (won't upgrade beyond this)
/// - Preferred quality (ideal target if available)
/// - Allowed quality items in order of preference
///
/// Design decision: We maintain an ordered list of allowed qualities
/// rather than just min/max to allow for complex preferences
/// (e.g., preferring BluRay over WEB-DL at the same resolution).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QualityProfile {
    /// Internal unique identifier
    pub id: Uuid,

    /// Profile name (e.g., "HD-1080p", "4K-Remux", "Any")
    pub name: String,

    /// Whether this profile should automatically upgrade
    /// when better quality becomes available
    pub upgrade_allowed: bool,

    /// Minimum acceptable quality
    /// Releases below this will be rejected
    pub min_quality: Quality,

    /// Maximum quality to pursue
    /// Releases above this won't be downloaded (even if available)
    pub max_quality: Quality,

    /// Preferred quality tier
    /// This is the "stop searching" quality - once we have this,
    /// we won't look for upgrades (unless a better quality becomes available)
    pub preferred_quality: Quality,
}

impl QualityProfile {
    /// Create a new quality profile
    pub fn new(name: String, min: Quality, max: Quality, preferred: Quality) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            upgrade_allowed: true,
            min_quality: min,
            max_quality: max,
            preferred_quality: preferred,
        }
    }

    /// Check if a quality is acceptable for this profile
    pub fn is_acceptable(&self, quality: &Quality) -> bool {
        quality >= &self.min_quality && quality <= &self.max_quality
    }

    /// Check if we should upgrade from current quality to new quality
    pub fn should_upgrade(&self, current: &Quality, candidate: &Quality) -> bool {
        if !self.upgrade_allowed {
            return false;
        }

        // Must be within acceptable range
        if !self.is_acceptable(candidate) {
            return false;
        }

        // Must be better than current
        if candidate <= current {
            return false;
        }

        // If we're at or above preferred, don't upgrade
        if current >= &self.preferred_quality {
            return false;
        }

        true
    }

    /// Check if we've reached the preferred quality
    pub fn is_preferred(&self, quality: &Quality) -> bool {
        quality >= &self.preferred_quality
    }

    /// Get a score for a quality within this profile
    ///
    /// Higher scores are better. Returns 0 for unacceptable qualities.
    /// This is useful for ranking multiple acceptable releases.
    pub fn score_quality(&self, quality: &Quality) -> i32 {
        if !self.is_acceptable(quality) {
            return 0;
        }

        let mut score = *quality as i32;

        // Bonus for preferred quality
        if quality == &self.preferred_quality {
            score += 1000;
        }

        score
    }
}

/// Individual quality item within a profile
///
/// This allows for fine-grained control over quality preferences,
/// including custom quality rankings and allowed/disallowed qualities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityItem {
    /// The quality tier
    pub quality: Quality,

    /// Whether this quality is allowed
    pub allowed: bool,

    /// Custom ranking within the profile (higher = more preferred)
    /// This allows overriding the default quality ordering
    pub rank: i32,
}

impl QualityItem {
    /// Create a new quality item
    pub fn new(quality: Quality, allowed: bool) -> Self {
        Self {
            quality,
            allowed,
            rank: quality as i32,
        }
    }

    /// Create an allowed quality item with default rank
    pub fn allowed(quality: Quality) -> Self {
        Self::new(quality, true)
    }

    /// Create a disallowed quality item
    pub fn disallowed(quality: Quality) -> Self {
        Self::new(quality, false)
    }
}

// Predefined quality profiles for common use cases

impl QualityProfile {
    /// Profile that accepts any quality (for testing or unconstrained downloads)
    pub fn any() -> Self {
        Self::new(
            "Any".to_string(),
            Quality::Sd480,
            Quality::Remux2160,
            Quality::Bluray1080,
        )
    }

    /// Profile targeting HD quality (720p-1080p)
    pub fn hd() -> Self {
        Self::new(
            "HD".to_string(),
            Quality::WebDl720,
            Quality::Bluray1080,
            Quality::Bluray1080,
        )
    }

    /// Profile targeting 1080p specifically
    pub fn hd_1080p() -> Self {
        Self::new(
            "HD-1080p".to_string(),
            Quality::WebDl1080,
            Quality::Remux1080,
            Quality::Bluray1080,
        )
    }

    /// Profile targeting 4K/2160p
    pub fn uhd_4k() -> Self {
        Self::new(
            "UHD-4K".to_string(),
            Quality::WebDl2160,
            Quality::Remux2160,
            Quality::Bluray2160,
        )
    }

    /// Profile for maximum quality (Remux only)
    pub fn remux() -> Self {
        Self::new(
            "Remux".to_string(),
            Quality::Remux1080,
            Quality::Remux2160,
            Quality::Remux2160,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_ordering() {
        assert!(Quality::Bluray1080 > Quality::WebDl1080);
        assert!(Quality::WebDl1080 > Quality::WebDl720);
        assert!(Quality::Remux2160 > Quality::Bluray2160);
    }

    #[test]
    fn test_quality_parsing() {
        assert_eq!(
            Quality::parse_from_release_name("Movie.2023.1080p.BluRay.x264"),
            Quality::Bluray1080
        );
        assert_eq!(
            Quality::parse_from_release_name("Show.S01E01.2160p.WEB-DL.H265"),
            Quality::WebDl2160
        );
        assert_eq!(
            Quality::parse_from_release_name("Film.1080p.REMUX.AVC"),
            Quality::Remux1080
        );
    }

    #[test]
    fn test_quality_profile_acceptance() {
        let profile = QualityProfile::hd();

        assert!(profile.is_acceptable(&Quality::WebDl720));
        assert!(profile.is_acceptable(&Quality::Bluray1080));
        assert!(!profile.is_acceptable(&Quality::Sd480));
        assert!(!profile.is_acceptable(&Quality::WebDl2160));
    }

    #[test]
    fn test_quality_profile_upgrade() {
        let profile = QualityProfile::hd_1080p();

        // Should upgrade from WEB-DL to BluRay
        assert!(profile.should_upgrade(&Quality::WebDl1080, &Quality::Bluray1080));

        // Should not upgrade beyond BluRay (our preferred)
        assert!(!profile.should_upgrade(&Quality::Bluray1080, &Quality::Remux1080));

        // Should not downgrade
        assert!(!profile.should_upgrade(&Quality::Bluray1080, &Quality::WebDl1080));
    }

    #[test]
    fn test_quality_resolution() {
        assert_eq!(Quality::Bluray1080.resolution(), Resolution::Hd1080);
        assert_eq!(Quality::WebDl720.resolution(), Resolution::Hd720);
        assert_eq!(Quality::Remux2160.resolution(), Resolution::Uhd2160);
    }
}
