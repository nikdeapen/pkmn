use pkmn_schema::cards::meta::EnergyType;

/// Converts an energy type `name` to an [EnergyType]. (ex: `Grass`)
pub fn energy_type(name: &str) -> Result<EnergyType, String> {
    Ok(match name.trim() {
        "Grass" => EnergyType::Grass,
        "Fire" => EnergyType::Fire,
        "Water" => EnergyType::Water,
        "Lightning" => EnergyType::Lightning,
        "Psychic" => EnergyType::Psychic,
        "Fighting" => EnergyType::Fighting,
        "Darkness" => EnergyType::Darkness,
        "Metal" => EnergyType::Metal,
        "Fairy" => EnergyType::Fairy,
        "Dragon" => EnergyType::Dragon,
        "Colorless" => EnergyType::Colorless,
        other => return Err(format!("unknown energy type: {other}")),
    })
}

/// Converts an energy `symbol` to an [EnergyType]. (ex: `{G}`)
pub fn energy_symbol(symbol: &str) -> Result<EnergyType, String> {
    Ok(match symbol.trim() {
        "{G}" => EnergyType::Grass,
        "{R}" => EnergyType::Fire,
        "{W}" => EnergyType::Water,
        "{L}" => EnergyType::Lightning,
        "{P}" => EnergyType::Psychic,
        "{F}" => EnergyType::Fighting,
        "{D}" => EnergyType::Darkness,
        "{M}" => EnergyType::Metal,
        "{Y}" => EnergyType::Fairy,
        "{N}" => EnergyType::Dragon,
        "{C}" => EnergyType::Colorless,
        other => return Err(format!("unknown energy symbol: {other}")),
    })
}
