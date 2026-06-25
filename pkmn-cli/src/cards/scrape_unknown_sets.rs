use pkmn_schema::cards::set::CardSet;
use pkmn_scrape::pkmncards::Client as PkmncardsClient;
use pkmn_scrape::pokemontcgio::Client as PokemontcgioClient;
use std::collections::BTreeMap;
use std::error::Error;

/// Scrapes the sets unknown to our local data from both sources and prints the aggregated,
/// de-duplicated list. (keyed by source id)
pub fn scrape_unknown_sets() -> Result<(), Box<dyn Error>> {
    let mut unknown: BTreeMap<String, CardSet> = BTreeMap::new();
    let pokemontcgio: Vec<CardSet> =
        PokemontcgioClient::with_local_cache().scrape_unknown_sets()?;
    let pkmncards: Vec<CardSet> = PkmncardsClient::with_local_cache().scrape_unknown_sets()?;
    for set in pokemontcgio.into_iter().chain(pkmncards) {
        unknown.insert(set.name().id().to_string(), set);
    }
    for set in unknown.values() {
        println!("{} ({})", set.name().id(), set.name().display());
    }
    println!("unknown: {}", unknown.len());
    Ok(())
}
