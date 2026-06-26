use pkmn_core::clean::PokemonNames;
use pkmn_core::validate::validate_unique_names;
use pkmn_schema::pokemon::Pokemon;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Gets the pokemon. (indexed by `dex - 1`)
#[must_use]
pub fn pokemon() -> &'static [Pokemon] {
    static LOCK: LazyLock<Vec<Pokemon>> = LazyLock::new(|| {
        let pokemon: &str = include_str!("../../data/pokemon.yml");
        let pokemon: Vec<Pokemon> = serde_yaml::from_str(pokemon).unwrap_or_else(|e| panic!("{e}"));
        validate(&pokemon).unwrap_or_else(|e| panic!("{e}"));
        pokemon
    });
    &LOCK
}

/// Gets the optional pokemon with the national `dex` number.
#[must_use]
pub fn pokemon_by_dex(dex: u32) -> Option<&'static Pokemon> {
    pokemon().get((dex as usize).checked_sub(1)?)
}

/// The pokemon keyed by `name.id`.
#[must_use]
pub fn pokemon_by_id() -> &'static HashMap<&'static str, &'static Pokemon> {
    static LOCK: LazyLock<HashMap<&'static str, &'static Pokemon>> =
        LazyLock::new(|| pokemon().iter().map(|p| (p.name().id(), p)).collect());
    &LOCK
}

/// Gets the [PokemonNames] index for cleaning card & evolution names.
#[must_use]
pub fn pokemon_names() -> &'static PokemonNames {
    static LOCK: LazyLock<PokemonNames> = LazyLock::new(|| PokemonNames::new(pokemon()));
    &LOCK
}

fn validate(pokemon: &[Pokemon]) -> Result<(), String> {
    for (i, p) in pokemon.iter().enumerate() {
        let expected_dex: u32 = (i + 1) as u32;
        if p.dex() != expected_dex {
            return Err(format!(
                "pokemon[{i}] has dex: {} (expected {expected_dex})",
                p.dex()
            ));
        }
    }
    validate_unique_names(pokemon.iter().map(Pokemon::name))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pokemon() {
        let all: &[Pokemon] = super::pokemon();
        assert_eq!(all.len(), 1025);
    }

    #[test]
    fn pokemon_by_id() {
        let map: &HashMap<&str, &Pokemon> = super::pokemon_by_id();
        assert_eq!(map.len(), 1025);
    }
}
