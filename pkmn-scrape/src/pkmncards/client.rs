use crate::util::local_cache;
use proto_packet::axum::http::HeaderMap;
use web_scrape::cache::WebCache;
use web_scrape::scrape::{ScrapeError, Scraper};
use web_scrape::source::WebSource;
use web_url::WebUrl;

/// A `pkmncards.com` scraping client.
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
    pub const URL_BASE: &str = "https://pkmncards.com";
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

    /// Scrapes the url `extension` with the `scrape` fn.
    pub fn scrape<T, F>(&self, extension: &str, scrape: F) -> Result<T, web_scrape::Error>
    where
        F: Fn(Scraper) -> Result<T, ScrapeError>,
    {
        let full: String = format!("{}/{}", Self::URL_BASE, extension);
        let url: WebUrl = full
            .parse()
            .map_err(|e| ScrapeError::Other(format!("invalid url '{}': {}", full, e)))?;
        self.source.scrape(&url, scrape)
    }
}
