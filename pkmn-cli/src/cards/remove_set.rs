use pkmn_scrape::pkmncards::set_id_map as pkmncards_set_id_map;
use pkmn_scrape::pokemontcgio::set_id_map as pokemontcgio_set_id_map;
use std::error::Error;

/// Removes the set `set_id` and marks its source as ignored for scraping in both sources.
pub fn remove_set(set_id: &str) -> Result<(), Box<dyn Error>> {
    pkmn_data::cards::remove_set(set_id)?;
    pkmncards_set_id_map::ignore(set_id)?;
    pokemontcgio_set_id_map::ignore(set_id)?;
    Ok(())
}
