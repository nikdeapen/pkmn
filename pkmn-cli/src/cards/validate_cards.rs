use crate::cards::print_errors;
use file_storage::{FilePath, FolderPath, StoragePath};
use pkmn_data::cards::validate_card;
use pkmn_schema::cards::card::Card;
use pkmn_schema::cards::set::CardSet;
use pkmn_scrape::resize_webp;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;

/// The R2 bucket base url for static assets.
const BUCKET: &str =
    "https://4a4bba3e2df525df01c99bae8307cbc5.r2.cloudflarestorage.com/pkmn-static";

/// The fixed small-image size & quality. (300×413, lossy WebP at 90%)
const SMALL_WIDTH: u32 = 300;
const SMALL_HEIGHT: u32 = 413;
const SMALL_QUALITY: f32 = 90.0;

/// Validates every card in the set `set_id`: the card data must be well-formed and both the large
/// & small images must exist. A missing small image is generated from the large; a missing large
/// image is recorded as an error. Cards are processed in parallel.
pub fn validate_set(set_id: &str) -> Result<(), Box<dyn Error>> {
    let report: SetReport = validate_set_report(set_id)?;
    report.print_summary();
    print_errors(&report.errors);
    Ok(())
}

/// Runs [validate_set] for every scraped set in the series `series_id`, then reports all errors
/// across the series together at the end.
pub fn validate_series(series_id: &str) -> Result<(), Box<dyn Error>> {
    let sets: Vec<&CardSet> = pkmn_data::cards::sets()
        .iter()
        .filter(|set| set.series().id() == series_id)
        .collect();
    if sets.is_empty() {
        return Err(format!("series not found: {series_id}").into());
    }
    let mut validated: usize = 0;
    let mut errors: Vec<String> = Vec::new();
    for set in sets {
        let set_id: &str = set.name().id();
        if pkmn_data::cards::cards_by_set().contains_key(set_id) {
            let report: SetReport = validate_set_report(set_id)?;
            report.print_summary();
            errors.extend(report.errors.iter().map(|error| format!("{set_id}: {error}")));
            validated += 1;
        } else {
            println!("{set_id}: no cards — skipping");
        }
    }
    if validated == 0 {
        return Err(format!("no scraped sets in series '{series_id}' — scrape it first").into());
    }
    print_errors(&errors);
    Ok(())
}

/// Validates the set's cards & images in parallel, returning the counts and error lines. (does not
/// print)
fn validate_set_report(set_id: &str) -> Result<SetReport, Box<dyn Error>> {
    let cards: &Vec<&Card> = pkmn_data::cards::cards_by_set()
        .get(set_id)
        .filter(|cards| !cards.is_empty())
        .ok_or_else(|| format!("no cards for set '{set_id}' — scrape it first"))?;
    let card_set: &CardSet = cards[0].set();
    let series: &str = card_set.series().id();
    let set: &str = card_set.name().id();

    // List existing images once per folder, then check membership in memory. (one R2 list each,
    // not a HEAD per card)
    let large_keys: HashSet<String> = image_keys("cards/en", series, set)?;
    let small_keys: HashSet<String> = image_keys("cards/small/en", series, set)?;

    // Process cards in parallel (download/resize/upload is IO-bound), collecting ordered outcomes.
    let threads: usize = std::thread::available_parallelism().map_or(4, |n| n.get()) * 2;
    let pool: rayon::ThreadPool = ThreadPoolBuilder::new().num_threads(threads).build()?;
    let outcomes: Vec<CardOutcome> = pool.install(|| {
        cards
            .par_iter()
            .map(|&card| process_card(card, series, set, &large_keys, &small_keys))
            .collect()
    });

    let mut generated: usize = 0;
    let mut errors: Vec<String> = Vec::new();
    for outcome in outcomes {
        if outcome.generated {
            generated += 1;
        }
        if let Some(error) = outcome.error {
            errors.push(error);
        }
    }
    Ok(SetReport {
        set_id: set_id.to_string(),
        cards: cards.len(),
        generated,
        errors,
    })
}

/// Validates one `card` and ensures its small image, returning the outcome. (image failures are
/// recorded as errors, never aborting the set)
fn process_card(
    card: &Card,
    series: &str,
    set: &str,
    large_keys: &HashSet<String>,
    small_keys: &HashSet<String>,
) -> CardOutcome {
    let key: String = card_image_key(card);
    if let Err(reason) = validate_card(card) {
        return CardOutcome::error(format!("invalid card {key}: {reason}"));
    }
    if !large_keys.contains(&key) {
        return CardOutcome::error(format!("missing large image: {key}"));
    }
    if small_keys.contains(&key) {
        return CardOutcome::default();
    }
    match generate_small(series, set, key.as_str()) {
        Ok(()) => CardOutcome::generated(),
        Err(e) => CardOutcome::error(format!("error generating small {key}: {e}")),
    }
}

/// Downloads the large image, resizes it to the small size, and uploads the small.
fn generate_small(series: &str, set: &str, key: &str) -> Result<(), Box<dyn Error>> {
    let large_data: Vec<u8> = image_path("cards/en", series, set, key)?.read_as_vec()?;
    let small_data: Vec<u8> = resize_webp(&large_data, SMALL_WIDTH, SMALL_HEIGHT, SMALL_QUALITY)?;
    image_path("cards/small/en", series, set, key)?.write_data(small_data)?;
    Ok(())
}

/// The card image key. (ex: `decidueye-ex-100`, the `{title}-{number}` slug)
fn card_image_key(card: &Card) -> String {
    let title: &str = card.title().id();
    let number: &str = card
        .number()
        .number()
        .map(|name| name.id())
        .unwrap_or_default();
    format!("{title}-{number}")
}

/// The keys of the existing card images under `prefix` for the set, from a single R2 listing.
fn image_keys(prefix: &str, series: &str, set: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let folder: FolderPath =
        StoragePath::parse(format!("{BUCKET}/{prefix}/{series}/{set}/"))?.to_folder()?;
    let mut keys: HashSet<String> = HashSet::new();
    for file in folder.list_files_as_vec()? {
        if let Some(key) = file.file_name().strip_suffix(".webp") {
            keys.insert(key.to_string());
        }
    }
    Ok(keys)
}

/// The R2 [FilePath] of a card image under `prefix`. (ex: `cards/en/{series}/{set}/{key}.webp`)
fn image_path(prefix: &str, series: &str, set: &str, key: &str) -> Result<FilePath, Box<dyn Error>> {
    Ok(StoragePath::parse(format!("{BUCKET}/{prefix}/{series}/{set}/{key}.webp"))?.to_file()?)
}

/// The result of validating one set.
struct SetReport {
    set_id: String,
    cards: usize,
    generated: usize,
    errors: Vec<String>,
}

impl SetReport {
    //! Display

    /// Prints the one-line per-set summary.
    fn print_summary(&self) {
        println!(
            "{}: {} cards · {} small generated · {} error(s)",
            self.set_id,
            self.cards,
            self.generated,
            self.errors.len()
        );
    }
}

/// The outcome of processing one card. (whether it generated a small + an error to report)
#[derive(Default)]
struct CardOutcome {
    generated: bool,
    error: Option<String>,
}

impl CardOutcome {
    //! Construction

    /// An outcome that recorded an `error`.
    fn error(error: String) -> Self {
        Self {
            generated: false,
            error: Some(error),
        }
    }

    /// An outcome that generated a small image.
    fn generated() -> Self {
        Self {
            generated: true,
            error: None,
        }
    }
}
