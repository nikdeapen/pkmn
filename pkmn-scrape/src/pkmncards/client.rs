use crate::pkmncards::set_id_map::IGNORE;
use crate::util::local_cache;
use proto_packet::axum::http::HeaderMap;
use std::collections::HashMap;
use std::sync::LazyLock;
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
    //! Set Id Map

    /// The set-id -> source-id map. (our set id -> `pkmncards.com` id, excludes ignored sources)
    ///
    /// Embedded at compile time; a set id absent from the map sources to itself.
    pub fn set_to_source(&self) -> &'static HashMap<&'static str, &'static str> {
        const CSV: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/pkmncards/sets/set_id_map.csv"
        ));
        static LOCK: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
            CSV.lines()
                .filter_map(|line| line.split_once(','))
                .map(|(source, set)| (source.trim(), set.trim()))
                .filter(|(_, set)| *set != IGNORE)
                .map(|(source, set)| (set, source))
                .collect()
        });
        &LOCK
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
