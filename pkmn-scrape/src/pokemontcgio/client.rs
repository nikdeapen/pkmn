use crate::util::local_cache;
use proto_packet::axum::http::HeaderMap;
use proto_packet::serde::de::DeserializeOwned;
use web_scrape::cache::WebCache;
use web_scrape::scrape::ScrapeError;
use web_scrape::source::WebSource;
use web_url::WebUrl;

/// A `pokemontcg.io` scraping client.
#[derive(Clone, Debug)]
pub struct Client {
    pub(super) source: WebSource,
}

impl Default for Client {
    fn default() -> Self {
        let cache: WebCache = WebCache::new(None, None);
        Self::with_cache(cache)
    }
}

impl Client {
    //! Constants

    /// The URL base.
    pub const URL_BASE: &str = "https://api.pokemontcg.io/v2/";
}

impl Client {
    //! Construction

    /// Creates a client with the `cache`.
    pub fn with_cache(cache: WebCache) -> Self {
        let headers: HeaderMap = HeaderMap::default();
        let source: WebSource = WebSource::new(headers, cache);
        Self { source }
    }

    /// Creates a client with the local cache.
    #[must_use]
    pub fn with_local_cache() -> Self {
        Self::with_cache(local_cache())
    }
}

impl Client {
    //! Scrape

    /// Scrapes the JSON encoded data from the `extension`.
    pub fn scrape<T>(&self, extension: &str) -> Result<T, web_scrape::Error>
    where
        T: DeserializeOwned,
    {
        let full: String = format!("{}{}", Self::URL_BASE, extension);
        let url: WebUrl = full
            .parse()
            .map_err(|e| ScrapeError::Other(format!("invalid url '{}': {}", full, e)))?;
        let content: String = self.source.get(&url)?;
        let value: T = serde_json::from_str(content.as_str())
            .map_err(|e| ScrapeError::Other(format!("invalid json from '{}': {}", url, e)))?;
        Ok(value)
    }
}
