use std::error::Error;

/// Validates the set data.
pub fn validate() -> Result<(), Box<dyn Error>> {
    pkmn_data::cards::validate()?;
    println!("sets are valid");
    Ok(())
}
