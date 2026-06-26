use include_dir::{Dir, File, include_dir};
use pkmn_core::cards::card_id;
use pkmn_schema::cards::card::Card;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::LazyLock;

/// The embedded English card data. (`data/cards/en/{series}/{set}.yml`)
static CARDS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../data/cards/en");

/// Gets all cards, ordered by set data file. (sorted by path)
#[must_use]
pub fn cards() -> &'static [Card] {
    static LOCK: LazyLock<Vec<Card>> = LazyLock::new(read_cards);
    &LOCK
}

/// The cards by [card_id]. (ex: `alakazam-base-set-1`)
#[must_use]
pub fn cards_by_id() -> &'static HashMap<String, &'static Card> {
    static LOCK: LazyLock<HashMap<String, &'static Card>> =
        LazyLock::new(|| cards().iter().map(|card| (card_id(card), card)).collect());
    &LOCK
}

/// The cards grouped by set id, in data-file order. (ex: `base-set`)
#[must_use]
pub fn cards_by_set() -> &'static HashMap<&'static str, Vec<&'static Card>> {
    static LOCK: LazyLock<HashMap<&'static str, Vec<&'static Card>>> = LazyLock::new(|| {
        let mut map: HashMap<&'static str, Vec<&'static Card>> = HashMap::new();
        for card in cards() {
            map.entry(card.set().name().id()).or_default().push(card);
        }
        map
    });
    &LOCK
}

/// Reads & deserializes the embedded card files, sorted by path for a stable order.
fn read_cards() -> Vec<Card> {
    let mut files: Vec<&'static File<'static>> = Vec::new();
    collect_files(&CARDS_DIR, &mut files);
    files.sort_by(|a, b| a.path().cmp(b.path()));
    let mut cards: Vec<Card> = Vec::new();
    for file in files {
        let path: &Path = file.path();
        if path.extension().and_then(OsStr::to_str) != Some("yml") {
            continue;
        }
        let yml: &str = file
            .contents_utf8()
            .unwrap_or_else(|| panic!("non-utf8 card file: {}", path.display()));
        let parsed: Vec<Card> = serde_yaml::from_str(yml)
            .unwrap_or_else(|e| panic!("invalid card file '{}': {e}", path.display()));
        cards.extend(parsed);
    }
    cards
}

/// Recursively collects the files in `dir` into `files`.
fn collect_files<'a>(dir: &'a Dir<'a>, files: &mut Vec<&'a File<'a>>) {
    files.extend(dir.files());
    for sub in dir.dirs() {
        collect_files(sub, files);
    }
}
