use pkmn_scrape::pkmncards::set_id_map;
use std::error::Error;

/// Removes the set `set_id` and marks its source as ignored for scraping.
pub fn remove_set(set_id: &str) -> Result<(), Box<dyn Error>> {
    pkmn_data::cards::remove_set(set_id)?;
    set_id_map::ignore(set_id)?;
    Ok(())
}
