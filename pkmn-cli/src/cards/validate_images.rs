use crate::cards::images::{KINDS, image_set_ids};
use std::collections::HashSet;
use std::error::Error;

/// Prints the sets missing a logo or symbol image in R2.
pub fn validate_images() -> Result<(), Box<dyn Error>> {
    let have: Vec<(&str, HashSet<String>)> = KINDS
        .iter()
        .map(|kind| Ok((*kind, image_set_ids(kind)?)))
        .collect::<Result<_, Box<dyn Error>>>()?;
    let mut missing: usize = 0;
    for set in pkmn_data::cards::sets() {
        for (kind, ids) in &have {
            if !ids.contains(set.name().id()) {
                println!("missing {kind}: {}", set.name().id());
                missing += 1;
            }
        }
    }
    println!("{missing} missing image(s)");
    Ok(())
}
