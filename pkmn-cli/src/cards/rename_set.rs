use crate::cards::images::{KINDS, copy_image};
use pkmn_scrape::pkmncards::set_id_map as pkmncards_set_id_map;
use pkmn_scrape::pokemontcgio::set_id_map as pokemontcgio_set_id_map;
use std::error::Error;

/// Renames the set `old_id` to `new_id`, updates the source-id mappings, and copies the set
/// images to the new id. (the originals are left in place for production)
pub fn rename_set(old_id: &str, new_id: &str) -> Result<(), Box<dyn Error>> {
    pkmn_data::cards::rename_set(old_id, new_id)?;
    pkmncards_set_id_map::update(old_id, new_id)?;
    pokemontcgio_set_id_map::update(old_id, new_id)?;
    for kind in KINDS {
        copy_image(kind, old_id, new_id, false)?;
    }
    Ok(())
}
