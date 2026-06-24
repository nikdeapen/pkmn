use crate::validate::{validate_display, validate_id};
use pkmn_schema::core::web::Name;

/// Validates the `name`.
pub fn validate_name(name: &Name) -> Result<(), String> {
    validate_id(name.id())?;
    validate_display(name.display())?;
    Ok(())
}
