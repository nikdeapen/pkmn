use crate::pkmncards::Client;
use pkmn_core::clean::display_to_name;
use pkmn_schema::cards::action::Action;
use pkmn_schema::cards::trainer::{TrainerCard, TrainerType};
use pkmn_schema::core::web::Name;
use web_scrape::scrape::{ScrapeError, Scraper};

impl Client {
    //! Scrape: Trainer Card

    /// Scrapes & cleans a Trainer card.
    pub(crate) fn scrape_trainer_card(&self, article: Scraper) -> Result<TrainerCard, ScrapeError> {
        let mut trainer: TrainerCard = TrainerCard::default();
        if let Some(hit_points) =
            self.scrape_optional_text(article, "div.name-hp-color span.hp a")?
        {
            let hit_points: &str = hit_points
                .strip_suffix(" HP")
                .unwrap_or(hit_points.as_str());
            let hit_points: u32 = hit_points.parse().map_err(|e| {
                ScrapeError::Other(format!("invalid hit points '{hit_points}': {e}"))
            })?;
            trainer.set_hit_points(hit_points);
        }
        // ex: fossil abilities & technical machine attacks are actions; the rest is plain text
        let mut texts: Vec<String> = Vec::new();
        let mut actions: Vec<Action> = Vec::new();
        for action in self.scrape_actions(article)? {
            match action {
                Action::Text(text) => {
                    if let Some(text) = text.text() {
                        texts.push(text.to_string());
                    }
                }
                action => actions.push(action),
            }
        }
        if !texts.is_empty() {
            trainer.set_text(texts.join("\n"));
        }
        if !actions.is_empty() {
            trainer.set_actions(actions);
        }
        let mut secondary_subs: Vec<Name> = Vec::new();
        for sub_type in self.scrape_texts(article, "div.type-evolves-is span.sub-type a")? {
            match self.trainer_type(sub_type.as_str()) {
                Some(trainer_type) => {
                    if trainer.sub().is_some() {
                        return Err(ScrapeError::Other("multiple trainer types".to_string()));
                    }
                    trainer.set_sub(trainer_type);
                }
                None => {
                    secondary_subs
                        .push(display_to_name(sub_type.as_str()).map_err(ScrapeError::Other)?);
                }
            }
        }
        if !secondary_subs.is_empty() {
            trainer.set_secondary_subs(secondary_subs);
        }
        Ok(trainer)
    }

    /// Gets the [TrainerType] for the `sub_type`, if any.
    #[must_use]
    fn trainer_type(&self, sub_type: &str) -> Option<TrainerType> {
        match sub_type {
            "Item" => Some(TrainerType::Item),
            "Stadium" => Some(TrainerType::Stadium),
            "Supporter" => Some(TrainerType::Supporter),
            "Pokémon Tool" => Some(TrainerType::PokemonTool),
            _ => None,
        }
    }
}
