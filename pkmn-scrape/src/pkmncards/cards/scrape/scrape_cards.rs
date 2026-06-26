use crate::pkmncards::Client;
use pkmn_core::clean::PokemonNames;
use pkmn_data::pokemon_names;
use pkmn_schema::cards::card::Card;
use pkmn_schema::cards::set::CardSet;
use web_scrape::Error;
use web_scrape::scrape::ScrapeError;

impl Client {
    //! Scrape: Cards

    /// Scrapes the cards for the `set`.
    pub fn scrape_cards(&self, set: &CardSet) -> Result<Vec<Card>, Error> {
        let set_id: &str = set.name().id();
        let set_id: &str = self.set_to_source().get(set_id).copied().unwrap_or(set_id);

        let pokemon: &PokemonNames = pokemon_names();
        let extension: String = format!("set/{set_id}/?sort=date&ord=auto&display=full");
        self.scrape(extension.as_str(), |page| {
            let cards: Vec<Card> = page.all("article.type-pkmn_card", |article| {
                self.scrape_card(article, pokemon)
            })?;
            if cards.is_empty() {
                Err(ScrapeError::Other(format!(
                    "no cards found for set '{set_id}'"
                )))
            } else {
                Ok(cards)
            }
        })
    }
}
