use crate::pkmncards::Client;
use pkmn_schema::cards::set::CardSet;
use std::collections::{BTreeMap, HashMap};
use web_scrape::scrape::ScrapeError;

impl Client {
    //! Scrape: Unknown Sets

    /// Scrapes the sets that are not in the local data.
    ///
    /// Scraped ids are mapped to our local ids via the set-id records so renamed sets
    /// are not reported as unknown.
    pub fn scrape_unknown_sets(&self) -> Result<Vec<CardSet>, web_scrape::Error> {
        let known: &HashMap<&str, Vec<&CardSet>> = pkmn_data::cards::sets_by_id();
        let source_to_set: BTreeMap<String, String> =
            super::set_id_map::source_to_set().map_err(|e| ScrapeError::Other(e.to_string()))?;
        let scraped: Vec<CardSet> = self.scrape_sets()?;
        Ok(scraped
            .into_iter()
            .filter(|set| {
                let source_id: &str = set.name().id();
                match source_to_set.get(source_id).map(String::as_str) {
                    Some(set_id) if set_id == super::set_id_map::IGNORE => false,
                    Some(set_id) => !known.contains_key(set_id),
                    None => !known.contains_key(source_id),
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn scrape_unknown_sets() {
        let client: Client = Client::with_local_cache();
        let unknown: Vec<CardSet> = client.scrape_unknown_sets().unwrap();
        println!("unknown: {:#?}", unknown);
    }
}
