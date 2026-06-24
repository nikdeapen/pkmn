use crate::pkmncards::Client;
use pkmn_core::clean::display_to_name;
use pkmn_schema::cards::set::{CardContext, CardSet};
use pkmn_schema::core::web::Name;
use scraper::ElementRef;
use web_scrape::scrape::{ScrapeError, Scraper};

impl Client {
    //! Scrape: Sets

    /// The URL for sets.
    const URL_SETS: &'static str = "/sets/";

    /// The set link prefix.
    const SET_PREFIX: &'static str = "/set/";

    /// The collection link prefix.
    const COLLECTION_PREFIX: &'static str = "/collection/";

    /// Scrapes the card sets.
    pub fn scrape_sets(&self) -> Result<Vec<CardSet>, web_scrape::Error> {
        self.scrape(Self::URL_SETS, |s| {
            let series: Vec<Vec<CardSet>> =
                s.all("div.entry-content > h2", |h| self.scrape_series(h))?;
            Ok(series.into_iter().flatten().collect())
        })
    }

    /// Scrapes the sets under the series heading `h2`.
    fn scrape_series(&self, h2: Scraper) -> Result<Vec<CardSet>, ScrapeError> {
        let series: Name = display_to_name(&h2.only_text("a")?).map_err(ScrapeError::from)?;
        let list: ElementRef = h2
            .element()
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .take_while(|e| e.value().name() != "h2")
            .find(|e| e.value().name() == "ul")
            .ok_or_else(|| format!("no set list for series: {}", series.display()))?;
        Scraper::from(list).all_flat("li", |li| self.scrape_set(&series, li))
    }

    /// Scrapes the set from the list item `li` in the `series`. Returns [None] for collections.
    fn scrape_set(&self, series: &Name, li: Scraper) -> Result<Option<CardSet>, ScrapeError> {
        let href: String = li.only_att("a", "href")?;
        let path: &str = href.strip_prefix(Self::URL_BASE).unwrap_or(&href);
        if path.starts_with(Self::COLLECTION_PREFIX) {
            return Ok(None);
        } else if !path.starts_with(Self::SET_PREFIX) {
            return Err(format!("unrecognized set link: {}", href).into());
        }
        let display: String = li.only_text("a")?;
        Ok(Some(self.name_to_set(series, &display)?))
    }

    /// Creates a [CardSet] in the `series` from the set `display`. (ex: `Chaos Rising (CRI)`)
    fn name_to_set(&self, series: &Name, display: &str) -> Result<CardSet, ScrapeError> {
        let (name, live_code): (Name, Option<Name>) = self.parse_set_name(display)?;
        Ok(CardSet::from(
            name,
            series.clone(),
            CardContext::English,
            live_code,
        ))
    }

    /// Parses the set `display` into its name and optional live code.
    /// (ex: `Chaos Rising (CRI)` -> `Chaos Rising` + `CRI`)
    fn parse_set_name(&self, display: &str) -> Result<(Name, Option<Name>), ScrapeError> {
        let display: &str = display.trim();
        if let Some(open) = display.strip_suffix(')').and_then(|d| d.rfind('(')) {
            let name: Name = display_to_name(display[..open].trim()).map_err(ScrapeError::from)?;
            let code: Name = display_to_name(display[open + 1..display.len() - 1].trim())
                .map_err(ScrapeError::from)?;
            Ok((name, Some(code)))
        } else {
            let name: Name = display_to_name(display).map_err(ScrapeError::from)?;
            Ok((name, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn scrape_sets() {
        let client: Client = Client::with_local_cache();
        let sets: Vec<CardSet> = client.scrape_sets().unwrap();
        println!("sets: {:#?}", sets);
    }
}
