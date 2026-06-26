use pkmn_schema::cards::card::Card;
use pkmn_schema::cards::set::CardSet;
use std::error::Error;
use std::path::{Path, PathBuf};

/// The English card data directory. (the workspace `data/cards/en`)
const CARDS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/cards/en");

/// Writes the `cards` for the `set` to `data/cards/en/{series}/{set}.yml`. (overwrites)
pub fn write_set_cards(set: &CardSet, cards: &[Card]) -> Result<(), Box<dyn Error>> {
    let path: PathBuf = Path::new(CARDS_DIR)
        .join(set.series().id())
        .join(format!("{}.yml", set.name().id()));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_yaml::to_string(cards)?)?;
    Ok(())
}
