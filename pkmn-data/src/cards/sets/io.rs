use pkmn_core::clean::display_to_name;
use pkmn_schema::cards::set::{CardContext, CardSet};
use pkmn_schema::core::web::Name;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

/// The set data file. (the workspace `data/cards/sets.yml`)
const SETS_YML: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/cards/sets.yml");

/// Writes the `sets` to [SETS_YML], grouped by context & series.
pub fn write_sets(sets: &[CardSet]) -> Result<(), Box<dyn Error>> {
    let sets: Vec<RawContext> = sets_to_contexts(sets);
    let sets: String = serde_yaml::to_string(&sets)?;
    if let Some(parent) = Path::new(SETS_YML).parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(SETS_YML, sets)?;
    Ok(())
}

/// Reads the sets, embedded from [SETS_YML] at compile time.
pub fn read_sets() -> Result<Vec<CardSet>, Box<dyn Error>> {
    let contexts: Vec<RawContext> =
        serde_yaml::from_str(include_str!("../../../../data/cards/sets.yml"))?;
    context_to_sets(&contexts)
}

/// Flattens the nested `contexts` into [CardSet]s.
fn context_to_sets(contexts: &[RawContext]) -> Result<Vec<CardSet>, Box<dyn Error>> {
    let mut sets: Vec<CardSet> = Vec::new();
    for context in contexts {
        for series in &context.series {
            let series_name: Name = Name::from(series.id.as_str(), series.display.as_str());
            for set in &series.sets {
                let name: Name = Name::from(set.id.as_str(), set.display.as_str());
                let live_code: Option<Name> =
                    set.live_code.as_deref().map(display_to_name).transpose()?;
                sets.push(CardSet::from(
                    name,
                    series_name.clone(),
                    context.context,
                    live_code,
                ));
            }
        }
    }
    Ok(sets)
}

/// Groups the `sets` into the nested context & series structure, preserving order.
fn sets_to_contexts(sets: &[CardSet]) -> Vec<RawContext> {
    fn index_or_push<T>(
        items: &mut Vec<T>,
        matches: impl Fn(&T) -> bool,
        new: impl FnOnce() -> T,
    ) -> usize {
        if let Some(index) = items.iter().position(matches) {
            return index;
        }
        items.push(new());
        items.len() - 1
    }
    let mut contexts: Vec<RawContext> = Vec::new();
    for set in sets {
        let c: usize = index_or_push(
            &mut contexts,
            |c| c.context == set.context(),
            || RawContext {
                context: set.context(),
                series: Vec::new(),
            },
        );
        let series: &mut Vec<RawSeries> = &mut contexts[c].series;
        let s: usize = index_or_push(
            series,
            |s| s.id == set.series().id(),
            || RawSeries {
                id: set.series().id().to_string(),
                display: set.series().display().to_string(),
                sets: Vec::new(),
            },
        );
        series[s].sets.push(RawSet {
            id: set.name().id().to_string(),
            display: set.name().display().to_string(),
            live_code: set.live_code().map(|code| code.display().to_string()),
        });
    }
    contexts
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RawContext {
    context: CardContext,
    series: Vec<RawSeries>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RawSeries {
    id: String,
    display: String,
    sets: Vec<RawSet>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RawSet {
    id: String,
    display: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    live_code: Option<String>,
}
