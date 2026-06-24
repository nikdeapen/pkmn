use crate::pkmncards::Client;
use pkmn_schema::cards::set::CardSet;
use std::collections::HashMap;

impl Client {
    //! Scrape: Unknown Sets

    /// Scrapes the sets that are not in the local data, comparing by `name.id`.
    pub fn scrape_unknown_sets(&self) -> Result<Vec<CardSet>, web_scrape::Error> {
        let known: &HashMap<&str, Vec<&CardSet>> = pkmn_data::cards::sets_by_id();
        let scraped: Vec<CardSet> = self.scrape_sets()?;
        Ok(scraped
            .into_iter()
            .filter(|set| !known.contains_key(set.name().id()))
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
