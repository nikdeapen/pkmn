use crate::pkmncards::Client;
use pkmn_core::clean::{energy_symbol, energy_type};
use pkmn_schema::cards::action::Action;
use pkmn_schema::cards::energy::{EnergyCard, SpecialEnergy};
use pkmn_schema::cards::meta::EnergyType;
use web_scrape::scrape::{ScrapeError, Scraper};

impl Client {
    //! Scrape: Energy Card

    /// Scrapes & cleans an Energy card.
    pub(crate) fn scrape_energy_card(&self, article: Scraper) -> Result<EnergyCard, ScrapeError> {
        let sub_types: Vec<String> =
            self.scrape_texts(article, "div.type-evolves-is span.sub-type a")?;
        if sub_types
            .iter()
            .any(|sub_type| sub_type == "Basic" || sub_type == "Basic Energy")
        {
            // ex: `Psychic Energy` & `Basic {P} Energy`
            let name: String = article
                .only_text("div.name-hp-color span.name a")?
                .trim()
                .to_string();
            let stripped: &str = name
                .strip_suffix(" Energy")
                .ok_or_else(|| ScrapeError::Other(format!("invalid basic energy name: {name}")))?;
            let stripped: &str = stripped.strip_prefix("Basic ").unwrap_or(stripped);
            let energy: EnergyType = if stripped.starts_with('{') {
                energy_symbol(stripped).map_err(ScrapeError::Other)?
            } else {
                energy_type(stripped).map_err(ScrapeError::Other)?
            };
            Ok(EnergyCard::Basic(energy))
        } else {
            let mut special: SpecialEnergy = SpecialEnergy::default();
            let mut texts: Vec<String> = Vec::new();
            for action in self.scrape_actions(article)? {
                match action {
                    Action::Text(text) => {
                        if let Some(text) = text.text() {
                            texts.push(text.to_string());
                        }
                    }
                    other => {
                        return Err(ScrapeError::Other(format!(
                            "unexpected action on energy card: {other:?}"
                        )));
                    }
                }
            }
            if !texts.is_empty() {
                special.set_text(texts.join("\n"));
            }
            Ok(EnergyCard::Special(special))
        }
    }
}
