use super::read_sets;
use pkmn_core::validate::validate_unique_names;
use pkmn_schema::cards::set::{CardContext, CardSet};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Gets the card sets. (in the order of the data file)
#[must_use]
pub fn sets() -> &'static [CardSet] {
    static LOCK: LazyLock<Vec<CardSet>> = LazyLock::new(|| {
        let sets: Vec<CardSet> = read_sets().unwrap_or_else(|e| panic!("{e}"));
        validate_sets(&sets).unwrap_or_else(|e| panic!("{e}"));
        sets
    });
    &LOCK
}

/// The sets grouped by `name.id`. (an id is shared by reprints, ex: energy sets)
#[must_use]
pub fn sets_by_id() -> &'static HashMap<&'static str, Vec<&'static CardSet>> {
    static LOCK: LazyLock<HashMap<&'static str, Vec<&'static CardSet>>> = LazyLock::new(|| {
        let mut map: HashMap<&'static str, Vec<&'static CardSet>> = HashMap::new();
        for set in sets() {
            map.entry(set.name().id()).or_default().push(set);
        }
        map
    });
    &LOCK
}

/// The sets grouped by context.
#[must_use]
pub fn sets_by_context() -> &'static HashMap<CardContext, Vec<&'static CardSet>> {
    static LOCK: LazyLock<HashMap<CardContext, Vec<&'static CardSet>>> = LazyLock::new(|| {
        let mut map: HashMap<CardContext, Vec<&'static CardSet>> = HashMap::new();
        for set in sets() {
            map.entry(set.context()).or_default().push(set);
        }
        map
    });
    &LOCK
}

/// Validates that the set ids are unique across all contexts.
fn validate_sets(sets: &[CardSet]) -> Result<(), String> {
    validate_unique_names(sets.iter().map(CardSet::name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets() {
        let all: &[CardSet] = super::sets();
        assert_eq!(all.len(), 193);
    }
}
