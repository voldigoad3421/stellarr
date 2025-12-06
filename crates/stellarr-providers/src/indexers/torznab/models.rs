//! Data models for Torznab/Newznab XML responses

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Root response wrapper for Torznab/Newznab RSS feed
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "rss")]
pub struct TorznabResponse {
    #[serde(rename = "@version")]
    pub version: String,
    pub channel: Channel,
}

/// RSS channel containing search results and metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    #[serde(rename = "item", default)]
    pub items: Vec<Item>,
    #[serde(rename = "image")]
    pub image: Option<Image>,
}

/// Channel image information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub link: String,
}

/// Individual search result item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub title: String,
    pub guid: Option<Guid>,
    #[serde(rename = "prowlarrindexer")]
    pub prowlarr_indexer: Option<ProwlarrIndexer>,
    pub link: Option<String>,
    #[serde(rename = "comments")]
    pub comments: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
    pub category: Option<Vec<String>>,
    pub description: Option<String>,
    pub enclosure: Option<Enclosure>,
    #[serde(rename = "attr", default)]
    pub attributes: Vec<TorznabAttr>,
}

/// GUID (globally unique identifier) for an item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guid {
    #[serde(rename = "@isPermaLink")]
    pub is_perma_link: Option<bool>,
    #[serde(rename = "$text")]
    pub value: String,
}

/// Prowlarr-specific indexer information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProwlarrIndexer {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub name: String,
}

/// Enclosure element containing download information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Enclosure {
    #[serde(rename = "@url")]
    pub url: String,
    #[serde(rename = "@length")]
    pub length: Option<String>,
    #[serde(rename = "@type")]
    pub content_type: Option<String>,
}

/// Torznab attribute (name-value pair for extended metadata)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TorznabAttr {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

impl Item {
    /// Get a specific Torznab attribute value by name
    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|attr| attr.name == name)
            .map(|attr| attr.value.as_str())
    }

    /// Get the size in bytes from attributes
    pub fn size(&self) -> Option<u64> {
        self.get_attr("size")
            .and_then(|s| s.parse::<u64>().ok())
    }

    /// Get seeders count from attributes
    pub fn seeders(&self) -> Option<u32> {
        self.get_attr("seeders")
            .and_then(|s| s.parse::<u32>().ok())
    }

    /// Get leechers/peers count from attributes
    pub fn leechers(&self) -> Option<u32> {
        self.get_attr("peers")
            .and_then(|s| s.parse::<u32>().ok())
    }

    /// Get category IDs from attributes
    pub fn category_ids(&self) -> Vec<u32> {
        self.attributes
            .iter()
            .filter(|attr| attr.name == "category")
            .filter_map(|attr| attr.value.parse::<u32>().ok())
            .collect()
    }

    /// Get the download URL (from enclosure or link)
    pub fn download_url(&self) -> Option<&str> {
        self.enclosure
            .as_ref()
            .map(|e| e.url.as_str())
            .or_else(|| self.link.as_deref())
    }

    /// Get the info URL (details page)
    pub fn info_url(&self) -> Option<&str> {
        self.get_attr("infoUrl")
            .or_else(|| self.comments.as_deref())
    }

    /// Get download volume factor (for freeleech detection)
    pub fn download_volume_factor(&self) -> f64 {
        self.get_attr("downloadvolumefactor")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(1.0)
    }

    /// Get upload volume factor
    pub fn upload_volume_factor(&self) -> f64 {
        self.get_attr("uploadvolumefactor")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(1.0)
    }

    /// Check if this is a freeleech torrent
    pub fn is_freeleech(&self) -> bool {
        self.download_volume_factor() == 0.0
    }

    /// Get IMDB ID from attributes
    pub fn imdb_id(&self) -> Option<&str> {
        self.get_attr("imdb")
            .or_else(|| self.get_attr("imdbid"))
    }

    /// Get TVDB ID from attributes
    pub fn tvdb_id(&self) -> Option<&str> {
        self.get_attr("tvdbid")
    }

    /// Get TMDB ID from attributes
    pub fn tmdb_id(&self) -> Option<&str> {
        self.get_attr("tmdbid")
    }

    /// Get TVRage ID from attributes
    pub fn tvrage_id(&self) -> Option<&str> {
        self.get_attr("rageid")
    }

    /// Parse publication date
    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        self.pub_date.as_ref().and_then(|date_str| {
            DateTime::parse_from_rfc2822(date_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        })
    }
}

/// Search response wrapper (alias for consistency)
pub type SearchResponse = TorznabResponse;

/// Capabilities response from indexer
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "caps")]
pub struct Capabilities {
    pub server: Option<ServerInfo>,
    pub limits: Option<Limits>,
    pub searching: Option<Searching>,
    pub categories: Option<Categories>,
}

/// Server information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInfo {
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@title")]
    pub title: Option<String>,
    #[serde(rename = "@strapline")]
    pub strapline: Option<String>,
    #[serde(rename = "@email")]
    pub email: Option<String>,
    #[serde(rename = "@url")]
    pub url: Option<String>,
    #[serde(rename = "@image")]
    pub image: Option<String>,
}

/// API rate limits
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Limits {
    #[serde(rename = "@max")]
    pub max: Option<String>,
    #[serde(rename = "@default")]
    pub default: Option<String>,
}

impl Limits {
    /// Get maximum results per query
    pub fn max_results(&self) -> Option<u32> {
        self.max.as_ref().and_then(|s| s.parse::<u32>().ok())
    }

    /// Get default results per query
    pub fn default_results(&self) -> Option<u32> {
        self.default.as_ref().and_then(|s| s.parse::<u32>().ok())
    }
}

/// Search capabilities
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Searching {
    pub search: Option<SearchCapability>,
    #[serde(rename = "tv-search")]
    pub tv_search: Option<SearchCapability>,
    #[serde(rename = "movie-search")]
    pub movie_search: Option<SearchCapability>,
    #[serde(rename = "music-search")]
    pub music_search: Option<SearchCapability>,
    #[serde(rename = "audio-search")]
    pub audio_search: Option<SearchCapability>,
    #[serde(rename = "book-search")]
    pub book_search: Option<SearchCapability>,
}

/// Individual search capability
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchCapability {
    #[serde(rename = "@available")]
    pub available: Option<String>,
    #[serde(rename = "@supportedParams")]
    pub supported_params: Option<String>,
}

impl SearchCapability {
    /// Check if this search type is available
    pub fn is_available(&self) -> bool {
        self.available
            .as_ref()
            .map(|s| s == "yes")
            .unwrap_or(false)
    }

    /// Get list of supported parameters
    pub fn params(&self) -> Vec<String> {
        self.supported_params
            .as_ref()
            .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
            .unwrap_or_default()
    }
}

/// Categories container
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Categories {
    #[serde(rename = "category", default)]
    pub categories: Vec<Category>,
}

/// Individual category
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "subcat", default)]
    pub subcategories: Vec<Category>,
}

impl Category {
    /// Get category ID as integer
    pub fn id_as_int(&self) -> Option<u32> {
        self.id.parse::<u32>().ok()
    }

    /// Recursively get all category and subcategory IDs
    pub fn all_ids(&self) -> Vec<u32> {
        let mut ids = vec![];
        if let Some(id) = self.id_as_int() {
            ids.push(id);
        }
        for subcat in &self.subcategories {
            ids.extend(subcat.all_ids());
        }
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_helpers() {
        let item = Item {
            title: "Test Release".to_string(),
            guid: None,
            prowlarr_indexer: None,
            link: Some("https://example.com/download".to_string()),
            comments: None,
            pub_date: None,
            category: None,
            description: None,
            enclosure: None,
            attributes: vec![
                TorznabAttr {
                    name: "size".to_string(),
                    value: "1073741824".to_string(),
                },
                TorznabAttr {
                    name: "seeders".to_string(),
                    value: "10".to_string(),
                },
                TorznabAttr {
                    name: "peers".to_string(),
                    value: "5".to_string(),
                },
                TorznabAttr {
                    name: "downloadvolumefactor".to_string(),
                    value: "0".to_string(),
                },
            ],
        };

        assert_eq!(item.size(), Some(1073741824));
        assert_eq!(item.seeders(), Some(10));
        assert_eq!(item.leechers(), Some(5));
        assert!(item.is_freeleech());
        assert_eq!(item.download_url(), Some("https://example.com/download"));
    }
}
