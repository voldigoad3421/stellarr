//! Quality and release information parser for torrent/usenet releases
//!
//! Extracts quality information from release titles using regex patterns.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Parsed release information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedRelease {
    /// Overall quality level (e.g., "1080p", "720p", "2160p")
    pub quality: Quality,
    /// Video resolution
    pub resolution: Resolution,
    /// Source type (e.g., BluRay, WEB-DL, HDTV)
    pub source: Source,
    /// Video codec (e.g., x264, x265, H.264)
    pub codec: Option<Codec>,
    /// HDR format if present
    pub hdr: Option<HdrFormat>,
    /// Whether this is a PROPER/REPACK release
    pub proper: bool,
    /// Whether this is a REPACK release
    pub repack: bool,
    /// Release group name
    pub release_group: Option<String>,
    /// Audio codec/format
    pub audio: Option<AudioFormat>,
    /// Edition information (Extended, Director's Cut, etc.)
    pub edition: Option<String>,
}

/// Quality level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Quality {
    Unknown,
    SDTV,
    DVD,
    WEBDL480p,
    HDTV720p,
    WEBDL720p,
    Bluray720p,
    HDTV1080p,
    WEBDL1080p,
    Bluray1080p,
    HDTV2160p,
    WEBDL2160p,
    Bluray2160p,
    BR_DISK,
    RAW_HD,
}

/// Resolution enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Resolution {
    Unknown,
    R480p,
    R576p,
    R720p,
    R1080p,
    R2160p,
}

/// Source type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    Unknown,
    CAM,
    TELESYNC,
    TELECINE,
    WORKPRINT,
    DVD,
    TV,
    WEBDL,
    WEBRip,
    BluRay,
    BluRayRIP,
    BDRip,
    BRRip,
    HDTV,
}

/// Video codec enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Codec {
    Unknown,
    XviD,
    X264,
    X265,
    H264,
    H265,
    HEVC,
    VP8,
    VP9,
    AV1,
}

/// HDR format enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HdrFormat {
    HDR,
    HDR10,
    HDR10Plus,
    DolbyVision,
    HLG,
}

/// Audio format enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioFormat {
    Unknown,
    MP3,
    AAC,
    AC3,
    DTS,
    DTSMA,
    DTSHD,
    TrueHD,
    FLAC,
    EAC3,
    Atmos,
}

// Regex patterns (compiled once at startup)
static RESOLUTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(480|576|720|1080|2160)[pi]\b").unwrap()
});

static QUALITY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(BluRay|Blu-Ray|HDTV|WEB-?DL|WEBRip|DVDRip|DVD|BDRip|BRRip|CAM|TELESYNC|TELECINE)\b").unwrap()
});

static CODEC_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b([xh]\.?26[45]|HEVC|XviD|AV1|VP[89])\b").unwrap()
});

static HDR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(HDR10\+|HDR10Plus|HDR10|HDR|DV|DoVi|Dolby\.?Vision|HLG)\b").unwrap()
});

static PROPER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(PROPER|REAL\.PROPER)\b").unwrap()
});

static REPACK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(REPACK|RERIP)\b").unwrap()
});

static RELEASE_GROUP_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"-([a-zA-Z0-9]+)(?:\[.*?\])?$").unwrap()
});

static AUDIO_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(AAC|AC3|DTS-HD|DTSHD|DTS-MA|DTSMA|DTS|TrueHD|FLAC|EAC3|DD\+?5\.1|Atmos|MP3)\b").unwrap()
});

static EDITION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(EXTENDED|UNRATED|DIRECTOR'?S?\.?CUT|THEATRICAL|REMASTERED|IMAX)\b").unwrap()
});

impl ParsedRelease {
    /// Parse quality information from a release title
    pub fn from_title(title: &str) -> Self {
        let resolution = Self::parse_resolution(title);
        let source = Self::parse_source(title);
        let quality = Self::determine_quality(resolution, source);
        let codec = Self::parse_codec(title);
        let hdr = Self::parse_hdr(title);
        let proper = PROPER_REGEX.is_match(title);
        let repack = REPACK_REGEX.is_match(title);
        let release_group = Self::parse_release_group(title);
        let audio = Self::parse_audio(title);
        let edition = Self::parse_edition(title);

        Self {
            quality,
            resolution,
            source,
            codec,
            hdr,
            proper,
            repack,
            release_group,
            audio,
            edition,
        }
    }

    /// Parse resolution from title
    fn parse_resolution(title: &str) -> Resolution {
        if let Some(cap) = RESOLUTION_REGEX.captures(title) {
            if let Some(res) = cap.get(1) {
                return match res.as_str() {
                    "480" => Resolution::R480p,
                    "576" => Resolution::R576p,
                    "720" => Resolution::R720p,
                    "1080" => Resolution::R1080p,
                    "2160" => Resolution::R2160p,
                    _ => Resolution::Unknown,
                };
            }
        }
        Resolution::Unknown
    }

    /// Parse source type from title
    fn parse_source(title: &str) -> Source {
        let title_lower = title.to_lowercase();

        if title_lower.contains("bluray") || title_lower.contains("blu-ray") {
            return Source::BluRay;
        }
        if title_lower.contains("web-dl") || title_lower.contains("webdl") {
            return Source::WEBDL;
        }
        if title_lower.contains("webrip") {
            return Source::WEBRip;
        }
        if title_lower.contains("hdtv") {
            return Source::HDTV;
        }
        if title_lower.contains("bdrip") {
            return Source::BDRip;
        }
        if title_lower.contains("brrip") {
            return Source::BRRip;
        }
        if title_lower.contains("dvdrip") {
            return Source::BluRayRIP;
        }
        if title_lower.contains("dvd") {
            return Source::DVD;
        }
        if title_lower.contains("cam") {
            return Source::CAM;
        }
        if title_lower.contains("telesync") || title_lower.contains("ts") {
            return Source::TELESYNC;
        }
        if title_lower.contains("telecine") || title_lower.contains("tc") {
            return Source::TELECINE;
        }

        Source::Unknown
    }

    /// Determine overall quality from resolution and source
    fn determine_quality(resolution: Resolution, source: Source) -> Quality {
        match (resolution, source) {
            (Resolution::R2160p, Source::BluRay) => Quality::Bluray2160p,
            (Resolution::R2160p, Source::WEBDL) => Quality::WEBDL2160p,
            (Resolution::R2160p, Source::HDTV) => Quality::HDTV2160p,
            (Resolution::R1080p, Source::BluRay) => Quality::Bluray1080p,
            (Resolution::R1080p, Source::WEBDL) => Quality::WEBDL1080p,
            (Resolution::R1080p, Source::HDTV) => Quality::HDTV1080p,
            (Resolution::R720p, Source::BluRay) => Quality::Bluray720p,
            (Resolution::R720p, Source::WEBDL) => Quality::WEBDL720p,
            (Resolution::R720p, Source::HDTV) => Quality::HDTV720p,
            (Resolution::R480p, Source::WEBDL) => Quality::WEBDL480p,
            (_, Source::DVD) => Quality::DVD,
            (_, Source::TV) => Quality::SDTV,
            _ => Quality::Unknown,
        }
    }

    /// Parse codec from title
    fn parse_codec(title: &str) -> Option<Codec> {
        if let Some(cap) = CODEC_REGEX.captures(title) {
            if let Some(codec) = cap.get(1) {
                let codec_str = codec.as_str().to_lowercase();
                return Some(match codec_str.as_str() {
                    s if s.contains("265") || s.contains("hevc") => Codec::H265,
                    s if s.contains("264") => Codec::H264,
                    "x265" => Codec::X265,
                    "x264" => Codec::X264,
                    "xvid" => Codec::XviD,
                    "av1" => Codec::AV1,
                    "vp9" => Codec::VP9,
                    "vp8" => Codec::VP8,
                    _ => Codec::Unknown,
                });
            }
        }
        None
    }

    /// Parse HDR format from title
    fn parse_hdr(title: &str) -> Option<HdrFormat> {
        if let Some(cap) = HDR_REGEX.captures(title) {
            if let Some(hdr) = cap.get(1) {
                let hdr_str = hdr.as_str().to_lowercase();
                return Some(match hdr_str.as_str() {
                    s if s.contains("hdr10+") || s.contains("hdr10plus") => HdrFormat::HDR10Plus,
                    s if s.contains("hdr10") => HdrFormat::HDR10,
                    s if s.contains("dv") || s.contains("dolby") || s.contains("vision") => {
                        HdrFormat::DolbyVision
                    }
                    s if s.contains("hlg") => HdrFormat::HLG,
                    s if s.contains("hdr") => HdrFormat::HDR,
                    _ => return None,
                });
            }
        }
        None
    }

    /// Parse release group from title
    fn parse_release_group(title: &str) -> Option<String> {
        if let Some(cap) = RELEASE_GROUP_REGEX.captures(title) {
            if let Some(group) = cap.get(1) {
                return Some(group.as_str().to_string());
            }
        }
        None
    }

    /// Parse audio format from title
    fn parse_audio(title: &str) -> Option<AudioFormat> {
        if let Some(cap) = AUDIO_REGEX.captures(title) {
            if let Some(audio) = cap.get(1) {
                let audio_str = audio.as_str().to_lowercase();
                return Some(match audio_str.as_str() {
                    s if s.contains("truehd") => AudioFormat::TrueHD,
                    s if s.contains("dts-hd") || s.contains("dtshd") => AudioFormat::DTSHD,
                    s if s.contains("dts-ma") || s.contains("dtsma") => AudioFormat::DTSMA,
                    s if s.contains("dts") => AudioFormat::DTS,
                    s if s.contains("atmos") => AudioFormat::Atmos,
                    s if s.contains("eac3") || s.contains("dd+") => AudioFormat::EAC3,
                    s if s.contains("ac3") => AudioFormat::AC3,
                    s if s.contains("aac") => AudioFormat::AAC,
                    s if s.contains("flac") => AudioFormat::FLAC,
                    s if s.contains("mp3") => AudioFormat::MP3,
                    _ => AudioFormat::Unknown,
                });
            }
        }
        None
    }

    /// Parse edition information from title
    fn parse_edition(title: &str) -> Option<String> {
        if let Some(cap) = EDITION_REGEX.captures(title) {
            if let Some(edition) = cap.get(1) {
                return Some(edition.as_str().to_string());
            }
        }
        None
    }

    /// Calculate a quality score for ranking releases
    ///
    /// Higher scores indicate better quality. This is useful for
    /// automatic quality selection and sorting.
    pub fn score(&self) -> u32 {
        let mut score = 0u32;

        // Base quality score (0-1000)
        score += match self.quality {
            Quality::Bluray2160p => 1000,
            Quality::WEBDL2160p => 950,
            Quality::HDTV2160p => 900,
            Quality::Bluray1080p => 800,
            Quality::WEBDL1080p => 750,
            Quality::HDTV1080p => 700,
            Quality::Bluray720p => 600,
            Quality::WEBDL720p => 550,
            Quality::HDTV720p => 500,
            Quality::WEBDL480p => 400,
            Quality::DVD => 300,
            Quality::SDTV => 200,
            Quality::BR_DISK => 150,
            Quality::RAW_HD => 100,
            Quality::Unknown => 0,
        };

        // Codec bonus
        score += match self.codec {
            Some(Codec::AV1) => 100,
            Some(Codec::H265) | Some(Codec::HEVC) | Some(Codec::X265) => 80,
            Some(Codec::H264) | Some(Codec::X264) => 60,
            Some(Codec::VP9) => 50,
            Some(Codec::VP8) => 30,
            Some(Codec::XviD) => 20,
            _ => 0,
        };

        // HDR bonus
        score += match self.hdr {
            Some(HdrFormat::DolbyVision) => 100,
            Some(HdrFormat::HDR10Plus) => 80,
            Some(HdrFormat::HDR10) => 60,
            Some(HdrFormat::HDR) => 50,
            Some(HdrFormat::HLG) => 40,
            None => 0,
        };

        // Audio bonus
        score += match self.audio {
            Some(AudioFormat::Atmos) => 80,
            Some(AudioFormat::TrueHD) => 70,
            Some(AudioFormat::DTSMA) => 65,
            Some(AudioFormat::DTSHD) => 60,
            Some(AudioFormat::DTS) => 50,
            Some(AudioFormat::EAC3) => 40,
            Some(AudioFormat::AC3) => 30,
            Some(AudioFormat::AAC) => 25,
            Some(AudioFormat::FLAC) => 20,
            Some(AudioFormat::MP3) => 10,
            _ => 0,
        };

        // PROPER/REPACK bonus
        if self.proper {
            score += 50;
        }
        if self.repack {
            score += 30;
        }

        score
    }

    /// Get a human-readable quality string
    pub fn quality_string(&self) -> String {
        let mut parts = vec![];

        match self.quality {
            Quality::Unknown => parts.push("Unknown"),
            Quality::SDTV => parts.push("SDTV"),
            Quality::DVD => parts.push("DVD"),
            Quality::WEBDL480p => parts.push("WEB-DL 480p"),
            Quality::HDTV720p => parts.push("HDTV 720p"),
            Quality::WEBDL720p => parts.push("WEB-DL 720p"),
            Quality::Bluray720p => parts.push("Bluray 720p"),
            Quality::HDTV1080p => parts.push("HDTV 1080p"),
            Quality::WEBDL1080p => parts.push("WEB-DL 1080p"),
            Quality::Bluray1080p => parts.push("Bluray 1080p"),
            Quality::HDTV2160p => parts.push("HDTV 2160p"),
            Quality::WEBDL2160p => parts.push("WEB-DL 2160p"),
            Quality::Bluray2160p => parts.push("Bluray 2160p"),
            Quality::BR_DISK => parts.push("BR-DISK"),
            Quality::RAW_HD => parts.push("Raw-HD"),
        }

        if let Some(codec) = &self.codec {
            parts.push(match codec {
                Codec::H264 | Codec::X264 => "H.264",
                Codec::H265 | Codec::HEVC | Codec::X265 => "H.265",
                Codec::AV1 => "AV1",
                Codec::VP9 => "VP9",
                Codec::VP8 => "VP8",
                Codec::XviD => "XviD",
                Codec::Unknown => "Unknown Codec",
            });
        }

        if let Some(hdr) = &self.hdr {
            parts.push(match hdr {
                HdrFormat::DolbyVision => "Dolby Vision",
                HdrFormat::HDR10Plus => "HDR10+",
                HdrFormat::HDR10 => "HDR10",
                HdrFormat::HDR => "HDR",
                HdrFormat::HLG => "HLG",
            });
        }

        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1080p_bluray() {
        let parsed = ParsedRelease::from_title(
            "Movie.Name.2023.1080p.BluRay.x264.DTS-HD.MA.5.1-SPARKS"
        );
        assert_eq!(parsed.resolution, Resolution::R1080p);
        assert_eq!(parsed.source, Source::BluRay);
        assert_eq!(parsed.quality, Quality::Bluray1080p);
        assert_eq!(parsed.codec, Some(Codec::X264));
        assert_eq!(parsed.audio, Some(AudioFormat::DTSHD));
        assert_eq!(parsed.release_group, Some("SPARKS".to_string()));
    }

    #[test]
    fn test_parse_2160p_web_hdr() {
        let parsed = ParsedRelease::from_title(
            "Show.S01E01.2160p.WEB-DL.H265.HDR10.DDP5.1-NTb"
        );
        assert_eq!(parsed.resolution, Resolution::R2160p);
        assert_eq!(parsed.source, Source::WEBDL);
        assert_eq!(parsed.quality, Quality::WEBDL2160p);
        assert_eq!(parsed.codec, Some(Codec::H265));
        assert_eq!(parsed.hdr, Some(HdrFormat::HDR10));
        assert_eq!(parsed.release_group, Some("NTb".to_string()));
    }

    #[test]
    fn test_parse_proper() {
        let parsed = ParsedRelease::from_title(
            "Movie.2023.1080p.BluRay.x264.PROPER-GROUP"
        );
        assert!(parsed.proper);
        assert!(!parsed.repack);
    }

    #[test]
    fn test_parse_repack() {
        let parsed = ParsedRelease::from_title(
            "Movie.2023.1080p.WEB-DL.REPACK-GROUP"
        );
        assert!(!parsed.proper);
        assert!(parsed.repack);
    }

    #[test]
    fn test_quality_scoring() {
        let bluray_4k = ParsedRelease::from_title(
            "Movie.2023.2160p.BluRay.x265.HDR10Plus.TrueHD.7.1.Atmos-GROUP"
        );
        let webdl_1080p = ParsedRelease::from_title(
            "Movie.2023.1080p.WEB-DL.H264.AAC-GROUP"
        );

        assert!(bluray_4k.score() > webdl_1080p.score());
    }

    #[test]
    fn test_quality_string() {
        let parsed = ParsedRelease::from_title(
            "Movie.2023.2160p.BluRay.x265.HDR10-GROUP"
        );
        let quality_str = parsed.quality_string();

        assert!(quality_str.contains("Bluray"));
        assert!(quality_str.contains("2160p"));
        assert!(quality_str.contains("H.265"));
        assert!(quality_str.contains("HDR10"));
    }
}
