use crate::clean::clean_display;
use crate::clean::display_to_id;
use crate::clean::display_to_name;
use pkmn_schema::core::web::Name;
use pkmn_schema::pokemon::Pokemon;
use std::collections::HashMap;

/// An index of the known Pokémon, used to clean card & evolution names.
pub struct PokemonNames {
    by_id: HashMap<String, &'static Pokemon>,
}

impl PokemonNames {
    //! Construction

    /// Creates a [PokemonNames] index from the `pokemon`.
    #[must_use]
    pub fn new(pokemon: &'static [Pokemon]) -> Self {
        let by_id: HashMap<String, &'static Pokemon> = pokemon
            .iter()
            .map(|pokemon| (pokemon.name().id().to_string(), pokemon))
            .collect();
        Self { by_id }
    }
}

impl PokemonNames {
    //! Names

    /// Cleans a card or evolution `name` into a [Name]. (ex: `Eevee`, `Dark Charizard`)
    pub fn card_name(&self, name: &str) -> Result<Name, String> {
        display_to_name(name)
    }

    /// Gets the known Pokémon for the `tag`, if any. (`None` when unknown, ex: fossils)
    pub fn pokemon(&self, tag: &str) -> Result<Option<&Pokemon>, String> {
        let id: String = display_to_id(&clean_display(tag)?)?;
        Ok(self.by_id.get(&id).copied())
    }
}
