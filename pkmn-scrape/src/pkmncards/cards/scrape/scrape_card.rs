use crate::pkmncards::Client;
use pkmn_core::clean::{PokemonNames, display_to_name};
use pkmn_schema::cards::card::{Card, CardNumber, CardType};
use pkmn_schema::cards::meta::CardMeta;
use pkmn_schema::cards::set::CardSet;
use pkmn_schema::core::web::Name;
use web_scrape::scrape::{ScrapeError, Scraper};

impl Client {
    //! Scrape: Card

    /// Scrapes & cleans one card from a set-page `article` element.
    pub(crate) fn scrape_card(
        &self,
        article: Scraper,
        set: &CardSet,
        pokemon: &PokemonNames,
    ) -> Result<Card, ScrapeError> {
        let title: Name = self.scrape_title(article, pokemon)?;
        let number: CardNumber = self.scrape_number(article)?;
        let meta: CardMeta = self.scrape_meta(article)?;
        let sub: CardType = self.scrape_sub(article, pokemon)?;
        Ok(Card::new(title, set.clone(), number, meta, sub))
    }

    /// Scrapes & cleans the card title. (its name, ex: `Eevee`)
    fn scrape_title(&self, article: Scraper, pokemon: &PokemonNames) -> Result<Name, ScrapeError> {
        let name: String = article
            .only_text("div.name-hp-color span.name a")?
            .trim()
            .to_string();
        pokemon.card_name(name.as_str()).map_err(ScrapeError::Other)
    }

    /// Scrapes & cleans the card number. (from the title, ex: `RC14`)
    ///
    /// The number comes from the title since the number taxonomy cannot represent symbol numbers.
    /// (ex: `Unown · Unseen Forces (UF) #!` has `XY149` for both `!` and `?`)
    fn scrape_number(&self, article: Scraper) -> Result<CardNumber, ScrapeError> {
        let title: String = article.only_text("h2.card-title a")?;
        let number: String = match title.rsplit_once('#') {
            Some((_, number)) if !number.trim().is_empty() => number.trim().to_string(),
            _ => article
                .only_text("div.release-meta span.number")?
                .trim()
                .to_string(),
        };
        let number: Name = display_to_name(number.as_str()).map_err(ScrapeError::Other)?;
        let mut card_number: CardNumber = CardNumber::default().with_number(number);
        if let Some(out_of) = self.scrape_optional_text(article, "div.release-meta span.out-of")? {
            let out_of: &str = out_of.strip_prefix('/').unwrap_or(out_of.as_str());
            card_number.set_printed_total(out_of.to_string());
        }
        Ok(card_number)
    }

    /// Dispatches to the card-type scraper. (`Pokémon`, `Trainer` or `Energy`)
    fn scrape_sub(
        &self,
        article: Scraper,
        pokemon: &PokemonNames,
    ) -> Result<CardType, ScrapeError> {
        let card_type: String = article
            .only_text("div.type-evolves-is span.type a")?
            .trim()
            .to_string();
        match card_type.as_str() {
            "Pokémon" => Ok(CardType::Pokemon(
                self.scrape_pokemon_card(article, pokemon)?,
            )),
            "Trainer" => Ok(CardType::Trainer(self.scrape_trainer_card(article)?)),
            "Energy" => Ok(CardType::Energy(self.scrape_energy_card(article)?)),
            other => Err(ScrapeError::Other(format!("unknown card type: {other}"))),
        }
    }
}

impl Client {
    //! Scrape: Utils

    /// Scrapes the trimmed texts of the `selection` elements.
    pub(crate) fn scrape_texts(
        &self,
        article: Scraper,
        selection: &str,
    ) -> Result<Vec<String>, ScrapeError> {
        article.all(selection, |s| {
            let text: String = s.element().text().collect();
            Ok(text.trim().to_string())
        })
    }

    /// Scrapes the trimmed text of the optional `selection`.
    pub(crate) fn scrape_optional_text(
        &self,
        article: Scraper,
        selection: &str,
    ) -> Result<Option<String>, ScrapeError> {
        let text: Option<String> = article.optional_text(selection)?;
        Ok(text.map(|text| text.trim().to_string()))
    }

    /// Scrapes the `title` attributes of the `selection` elements.
    pub(crate) fn scrape_title_atts(
        &self,
        article: Scraper,
        selection: &str,
    ) -> Result<Vec<String>, ScrapeError> {
        article.all(selection, |s| {
            s.element()
                .attr("title")
                .map(String::from)
                .ok_or_else(|| ScrapeError::Other(format!("'{selection}' missing 'title'")))
        })
    }
}
