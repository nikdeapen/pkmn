use crate::validate::validate_name;
use pkmn_schema::core::web::Name;
use std::collections::HashSet;

/// Validates each of the `names` and ensures that each of the `names` has a unique `id`.
pub fn validate_unique_names<'a, I>(names: I) -> Result<(), String>
where
    I: Iterator<Item = &'a Name>,
{
    let mut ids: HashSet<&str> = HashSet::default();
    for name in names {
        validate_name(name)?;
        if !ids.insert(name.id()) {
            return Err(format!("duplicate id string: {}", name.id()));
        }
    }
    Ok(())
}
