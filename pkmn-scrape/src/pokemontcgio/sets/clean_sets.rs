use crate::pokemontcgio::client::Client;
use crate::pokemontcgio::model::RawSet;
use pkmn_core::clean::display_to_name;
use pkmn_schema::cards::set::{CardContext, CardSet};
use pkmn_schema::core::web::Name;

impl Client {
    //! Clean Sets

    /// Converts the scraped raw `sets` into [CardSet]s.
    pub fn clean_sets(&self, sets: &[RawSet]) -> Result<Vec<CardSet>, String> {
        sets.iter().map(|set| self.clean_set(set)).collect()
    }

    /// Converts the scraped raw `set` into a [CardSet].
    pub(crate) fn clean_set(&self, set: &RawSet) -> Result<CardSet, String> {
        let name: Name = display_to_name(&set.name)?;
        let series: Name = display_to_name(&set.series)?;
        let live_code: Option<Name> = match set.ptcgo_code.as_deref() {
            Some(code) => Some(display_to_name(code)?),
            None => None,
        };
        Ok(CardSet::new(name, series, CardContext::English, live_code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn clean_sets() {
        let client: Client = Client::with_local_cache();
        let sets: Vec<RawSet> = client.scrape_sets().unwrap();
        let sets: Vec<CardSet> = client.clean_sets(&sets).unwrap();
        for set in &sets {
            println!("{:?}", set);
        }
        println!("cleaned {} sets", sets.len());
    }
}
