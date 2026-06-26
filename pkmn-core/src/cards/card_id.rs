use pkmn_schema::cards::card::Card;

/// Builds the unique id for the `card`. (ex: `alakazam-base-set-1`)
#[must_use]
pub fn card_id(card: &Card) -> String {
    let title: &str = card.title().id();
    let set: &str = card.set().name().id();
    let number: &str = card
        .number()
        .number()
        .map(|name| name.id())
        .unwrap_or_default();
    format!("{title}-{set}-{number}")
}
