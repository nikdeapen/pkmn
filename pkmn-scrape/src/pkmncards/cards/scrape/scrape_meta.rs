use crate::pkmncards::Client;
use pkmn_core::clean::display_to_name;
use pkmn_schema::cards::meta::CardMeta;
use pkmn_schema::core::web::Name;
use proto_packet::types::Date;
use web_scrape::scrape::{ScrapeError, Scraper};

impl Client {
    //! Scrape: Meta

    /// Scrapes & cleans the card metadata. (artists, level, rarity, release date & flavor)
    pub(crate) fn scrape_meta(&self, article: Scraper) -> Result<CardMeta, ScrapeError> {
        let mut meta: CardMeta = CardMeta::default();
        let artists: Vec<Name> = self.scrape_artists(article)?;
        if !artists.is_empty() {
            meta.set_artists(artists);
        }
        if let Some(level) = self.scrape_level(article)? {
            meta.set_level(level);
        }
        meta.set_rarity(self.scrape_rarity(article)?);
        meta.set_release_date(self.scrape_release_date(article)?);
        if let Some(flavor) = self.scrape_flavor(article)? {
            meta.set_flavor(flavor);
        }
        Ok(meta)
    }

    /// Scrapes & cleans the illustrators. (empty when unattributed)
    fn scrape_artists(&self, article: Scraper) -> Result<Vec<Name>, ScrapeError> {
        self.scrape_texts(article, r#"div.illus a[href*="/artist/"]"#)?
            .iter()
            .map(|artist| display_to_name(artist.as_str()))
            .collect::<Result<_, String>>()
            .map_err(ScrapeError::Other)
    }

    /// Scrapes & cleans the level. (ex: `LV.42` -> `42`)
    fn scrape_level(&self, article: Scraper) -> Result<Option<Name>, ScrapeError> {
        match self.scrape_optional_text(article, "div.illus span.level a")? {
            None => Ok(None),
            Some(level) => {
                let level: &str = level.strip_prefix("LV.").unwrap_or(level.as_str());
                Ok(Some(display_to_name(level).map_err(ScrapeError::Other)?))
            }
        }
    }

    /// Scrapes & cleans the rarity. (ex: `Rare Holo`, `No Rarity`)
    fn scrape_rarity(&self, article: Scraper) -> Result<Name, ScrapeError> {
        let rarity: String = article
            .only_text("div.release-meta span.rarity")?
            .trim()
            .to_string();
        display_to_name(rarity.as_str()).map_err(ScrapeError::Other)
    }

    /// Scrapes & cleans the release date. (ex: `↘ Jan 9, 1999`)
    fn scrape_release_date(&self, article: Scraper) -> Result<Date, ScrapeError> {
        let date: String = article.only_text("div.release-meta span.date")?;
        let date: &str = date.trim().trim_start_matches('↘').trim();
        Date::parse_from_str(date, "%b %-d, %Y")
            .map_err(|e| ScrapeError::Other(format!("invalid release date '{date}': {e}")))
    }

    /// Scrapes the flavor text.
    fn scrape_flavor(&self, article: Scraper) -> Result<Option<String>, ScrapeError> {
        self.scrape_optional_text(article, "div.flavor")
    }
}
