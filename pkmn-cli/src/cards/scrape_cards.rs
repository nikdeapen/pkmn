use crate::cards::print_errors;
use pkmn_schema::cards::card::Card;
use pkmn_schema::cards::set::CardSet;
use pkmn_scrape::pkmncards::Client as PkmncardsClient;
use std::error::Error;

/// Scrapes & writes all cards for the set `set_id` from `source`. (only `pkmncards` for now)
pub fn scrape_set(source: &str, set_id: &str) -> Result<(), Box<dyn Error>> {
    let client: PkmncardsClient = pkmncards_client(source)?;
    let set: &CardSet = find_set(set_id)?;
    let count: usize = scrape_and_write(&client, set)?;
    println!("{} ({}): {count} cards", set.name().id(), set.series().id());
    Ok(())
}

/// Scrapes & writes all cards for every set in the series `series_id` from `source`. A set that
/// fails (ex: not on the source — a `404`) is recorded and reported at the end, not fatal.
pub fn scrape_series(source: &str, series_id: &str) -> Result<(), Box<dyn Error>> {
    let client: PkmncardsClient = pkmncards_client(source)?;
    let sets: Vec<&CardSet> = pkmn_data::cards::sets()
        .iter()
        .filter(|set| set.series().id() == series_id)
        .collect();
    if sets.is_empty() {
        return Err(format!("series not found: {series_id}").into());
    }
    let mut errors: Vec<String> = Vec::new();
    for set in sets {
        let set_id: &str = set.name().id();
        match scrape_and_write(&client, set) {
            Ok(count) => println!("{set_id} ({}): {count} cards", set.series().id()),
            Err(e) => errors.push(format!("{set_id}: {e}")),
        }
    }
    print_errors(&errors);
    Ok(())
}

/// Scrapes the cards for one `set` from `client` and writes them, returning the card count.
fn scrape_and_write(client: &PkmncardsClient, set: &CardSet) -> Result<usize, Box<dyn Error>> {
    let cards: Vec<Card> = client.scrape_cards(set)?;
    pkmn_data::cards::write_set_cards(set, &cards)?;
    Ok(cards.len())
}

/// Resolves the scrape `source` to a client. (only `pkmncards` is supported for now)
fn pkmncards_client(source: &str) -> Result<PkmncardsClient, Box<dyn Error>> {
    match source {
        "pkmncards" => Ok(PkmncardsClient::with_local_cache()),
        other => Err(format!("unsupported source: '{other}' (only 'pkmncards')").into()),
    }
}

/// Finds the set with `set_id` in the local set data.
fn find_set(set_id: &str) -> Result<&'static CardSet, Box<dyn Error>> {
    pkmn_data::cards::sets()
        .iter()
        .find(|set| set.name().id() == set_id)
        .ok_or_else(|| format!("set not found: {set_id}").into())
}
