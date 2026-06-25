use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

/// The set-id map file. (source `pkmncards.com` id -> our set id, or [IGNORE])
const SET_ID_MAP_CSV: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/pkmncards/sets/set_id_map.csv"
);

/// The set id marking a source set we don't track locally.
pub const IGNORE: &str = "IGNORE";

/// Reads the source-id -> set-id map. Empty if the file is absent.
pub fn read() -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    if !Path::new(SET_ID_MAP_CSV).exists() {
        return Ok(BTreeMap::new());
    }
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for line in std::fs::read_to_string(SET_ID_MAP_CSV)?.lines() {
        if let Some((source, set)) = line.split_once(',') {
            map.insert(source.trim().to_string(), set.trim().to_string());
        }
    }
    Ok(map)
}

/// Writes the source-id -> set-id `map`.
pub fn write(map: &BTreeMap<String, String>) -> Result<(), Box<dyn Error>> {
    let csv: String = map
        .iter()
        .map(|(source, set)| format!("{source},{set}\n"))
        .collect();
    std::fs::write(SET_ID_MAP_CSV, csv)?;
    Ok(())
}

/// Updates the map for a set renamed from `old_set_id` to `new_set_id`. The entry is
/// dropped when the set id returns to its source id.
pub fn update(old_set_id: &str, new_set_id: &str) -> Result<(), Box<dyn Error>> {
    let mut map: BTreeMap<String, String> = read()?;
    let source: String = source_of(&map, old_set_id);
    if new_set_id == source {
        map.remove(&source);
    } else {
        map.insert(source, new_set_id.to_string());
    }
    write(&map)
}

/// Marks the source of `set_id` as [IGNORE]d, so it is not reported as unknown.
pub fn ignore(set_id: &str) -> Result<(), Box<dyn Error>> {
    let mut map: BTreeMap<String, String> = read()?;
    let source: String = source_of(&map, set_id);
    map.insert(source, IGNORE.to_string());
    write(&map)
}

/// The source-id -> set-id map.
pub fn source_to_set() -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    read()
}

/// The set-id -> source-id map. (excludes ignored sources)
pub fn set_to_source() -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    Ok(read()?
        .into_iter()
        .filter(|(_, set)| set != IGNORE)
        .map(|(source, set)| (set, source))
        .collect())
}

/// The source id currently mapped to `set_id`, or `set_id` itself.
fn source_of(map: &BTreeMap<String, String>, set_id: &str) -> String {
    map.iter()
        .find(|(_, set)| set.as_str() == set_id)
        .map(|(source, _)| source.clone())
        .unwrap_or_else(|| set_id.to_string())
}
