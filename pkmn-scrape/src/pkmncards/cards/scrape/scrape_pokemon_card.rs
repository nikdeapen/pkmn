use crate::pkmncards::Client;
use pkmn_core::clean::{PokemonNames, clean_modifier_value, display_to_name, energy_type};
use pkmn_schema::cards::action::Action;
use pkmn_schema::cards::meta::EnergyType;
use pkmn_schema::cards::pokemon::{Evolution, PokemonCard, TypeModifier};
use pkmn_schema::core::web::Name;
use pkmn_schema::pokemon::Pokemon;
use web_scrape::scrape::{ScrapeError, Scraper};

impl Client {
    //! Scrape: Pokemon Card

    /// Scrapes & cleans a Pokémon card.
    pub(crate) fn scrape_pokemon_card(
        &self,
        article: Scraper,
        pokemon: &PokemonNames,
    ) -> Result<PokemonCard, ScrapeError> {
        // fossil pokemon cards have no pokemon (ex: `Buried Fossil`)
        let pokemons: Vec<Pokemon> = self
            .scrape_texts(article, "div.type-evolves-is span.pokemon a")?
            .iter()
            .map(|tag| {
                let found: Option<&Pokemon> =
                    pokemon.pokemon(tag.as_str()).map_err(ScrapeError::Other)?;
                found
                    .cloned()
                    .ok_or_else(|| ScrapeError::Other(format!("unknown pokemon: {tag}")))
            })
            .collect::<Result<_, ScrapeError>>()?;
        let hit_points: u32 = self
            .scrape_hit_points(article)?
            .ok_or_else(|| ScrapeError::Other("missing hit points".to_string()))?;
        let energy_types: Vec<EnergyType> = self
            .scrape_title_atts(article, "div.name-hp-color span.color abbr")?
            .iter()
            .map(|color| energy_type(color.as_str()))
            .collect::<Result<_, String>>()
            .map_err(ScrapeError::Other)?;
        let actions: Vec<Action> = self.scrape_actions(article)?;
        let mut card: PokemonCard = PokemonCard::default()
            .with_pokemon(pokemons)
            .with_hit_points(hit_points)
            .with_energy_types(energy_types)
            .with_evolution(self.scrape_evolution(article, pokemon)?)
            .with_actions(actions);
        if let Some(retreat) =
            self.scrape_optional_text(article, "div.weak-resist-retreat span.retreat abbr")?
        {
            let retreat: u32 = retreat.parse().map_err(|e| {
                ScrapeError::Other(format!("invalid retreat cost '{retreat}': {e}"))
            })?;
            card.set_retreat_cost(retreat);
        }
        if let Some(weakness) =
            self.scrape_type_modifier(article, "div.weak-resist-retreat span.weak")?
        {
            card.set_weaknesses(vec![weakness]);
        }
        if let Some(resistance) =
            self.scrape_type_modifier(article, "div.weak-resist-retreat span.resist")?
        {
            card.set_resistances(vec![resistance]);
        }
        Ok(card)
    }

    /// Scrapes & cleans the hit points. (ex: `60 HP` -> `60`)
    fn scrape_hit_points(&self, article: Scraper) -> Result<Option<u32>, ScrapeError> {
        let hit_points: Option<String> =
            self.scrape_optional_text(article, "div.name-hp-color span.hp a")?;
        match hit_points {
            None => Ok(None),
            Some(hit_points) => {
                let hit_points: &str = hit_points
                    .strip_suffix(" HP")
                    .unwrap_or(hit_points.as_str());
                let hit_points: u32 = hit_points.parse().map_err(|e| {
                    ScrapeError::Other(format!("invalid hit points '{hit_points}': {e}"))
                })?;
                Ok(Some(hit_points))
            }
        }
    }

    /// Scrapes & cleans the evolution. (stage & evolves-from)
    fn scrape_evolution(
        &self,
        article: Scraper,
        pokemon: &PokemonNames,
    ) -> Result<Evolution, ScrapeError> {
        let mut evolution: Evolution = Evolution::default();
        if let Some(stage) =
            self.scrape_optional_text(article, "div.type-evolves-is span.stage a")?
        {
            evolution.set_stage(display_to_name(stage.as_str()).map_err(ScrapeError::Other)?);
        }
        if let Some(evolves) =
            self.scrape_optional_text(article, "div.type-evolves-is span.evolves")?
        {
            if let Some(rest) = evolves.strip_prefix("Evolves from ") {
                // the from pokemon may be missing (ex: `Evolves from and into Typhlosion`)
                let from: &str = rest.split(" and into ").next().unwrap_or(rest).trim();
                if !from.is_empty() && !from.starts_with("and into ") {
                    let from: Name = pokemon.card_name(from).map_err(ScrapeError::Other)?;
                    evolution.set_evolves_from(vec![from]);
                }
            } else if let Some(onto) = evolves.strip_prefix("Put onto ") {
                // LV.X cards level-up the pokemon they are put onto (ex: `Put onto Zacian`)
                let onto: Name = pokemon.card_name(onto).map_err(ScrapeError::Other)?;
                evolution.set_evolves_from(vec![onto]);
            } else if !evolves.starts_with("Evolves into ") {
                return Err(ScrapeError::Other(format!(
                    "invalid evolves text: {evolves}"
                )));
            }
        }
        Ok(evolution)
    }

    /// Scrapes & cleans the optional weakness/resistance modifier at the `selection`.
    ///
    /// Returns `None` for missing and `No Weakness`/`No Resistance` modifiers.
    fn scrape_type_modifier(
        &self,
        article: Scraper,
        selection: &str,
    ) -> Result<Option<TypeModifier>, ScrapeError> {
        let modifier: Option<(Vec<String>, Vec<String>)> = article.optional(selection, |span| {
            let energy_types: Vec<String> = self.scrape_title_atts(span, "abbr")?;
            let modifiers: Vec<String> = self.scrape_texts(span, r#"span[title$="Modifier"]"#)?;
            Ok((energy_types, modifiers))
        })?;
        let (energy_types, modifiers): (Vec<String>, Vec<String>) = match modifier {
            Some(modifier) => modifier,
            None => return Ok(None),
        };
        if energy_types.iter().all(|energy| energy.starts_with("No ")) {
            return Ok(None);
        }
        let energy_types: Vec<EnergyType> = energy_types
            .iter()
            .map(|energy| energy_type(energy.as_str()))
            .collect::<Result<_, String>>()
            .map_err(ScrapeError::Other)?;
        let value: &str = match modifiers.as_slice() {
            [value] => value,
            values => {
                return Err(ScrapeError::Other(format!(
                    "expected 1 modifier value: {values:?}"
                )));
            }
        };
        let (scalar, damage_modifier) = clean_modifier_value(value).map_err(ScrapeError::Other)?;
        Ok(Some(
            TypeModifier::default()
                .with_energy_types(energy_types)
                .with_scalar(scalar)
                .with_modifier(damage_modifier),
        ))
    }
}
