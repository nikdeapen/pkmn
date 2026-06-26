use pkmn_core::validate::validate_name;
use pkmn_schema::cards::card::Card;

/// Validates a single `card`: its title must be a well-formed name and it must have a valid number.
pub fn validate_card(card: &Card) -> Result<(), String> {
    validate_name(card.title())?;
    match card.number().number() {
        Some(number) => validate_name(number),
        None => Err("missing card number".to_string()),
    }
}
