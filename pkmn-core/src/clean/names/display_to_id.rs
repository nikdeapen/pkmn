use crate::validate::{validate_display, validate_id};

/// Converts a display string to an id string.
pub fn display_to_id(display: &str) -> Result<String, String> {
    debug_assert!(validate_display(display).is_ok());

    const FILTER: &str = "'.!#\"[]?{}(),:";
    const MAP: &[(char, &str)] = &[
        (' ', "-"),
        ('_', "blank"),
        ('é', "e"),
        ('α', "alpha"),
        ('β', "beta"),
        ('γ', "gamma"),
        ('δ', "delta"),
        ('Δ', "delta"),
        ('θ', "theta"),
        ('Ω', "omega"),
        ('−', "minus"),
        ('+', "plus"),
        ('♂', "male"),
        ('♀', "female"),
        ('★', "star"),
        ('☆', "star"),
        ('◇', "prism-star"),
    ];

    // a run of underscores reads as a single blank (ex: `_____'s Pikachu` -> `blanks-pikachu`)
    let mut display: String = display.replace(" & ", " ");
    while display.contains("__") {
        display = display.replace("__", "_");
    }
    let mut result: String = String::with_capacity(display.len());

    for c in display.chars() {
        match c {
            c if c.is_ascii_uppercase() => result.push(c.to_ascii_lowercase()),
            c if c.is_ascii_lowercase() | c.is_ascii_digit() => result.push(c),
            c if FILTER.contains(c) => {}
            c if is_cjk(c) => {}
            c if MAP.iter().map(|(c, _)| c).any(|mc| c == *mc) => MAP
                .iter()
                .find(|(mc, _)| c == *mc)
                .iter()
                .for_each(|(_, s)| result.push_str(s)),
            c => result.push(c),
        }
    }

    validate_id(&result)?;

    Ok(result)
}

/// Whether `c` is a CJK character (hiragana, katakana, ideographs, or full/half-width forms).
///
/// pokemontcg.io occasionally pollutes English card names with the Japanese name
/// (ex: `ナッシー[Exeggutor]`); dropping these keeps the id formed from the English text.
#[must_use]
fn is_cjk(c: char) -> bool {
    matches!(
        c,
        '\u{3000}'..='\u{30ff}' | '\u{3400}'..='\u{4dbf}' | '\u{4e00}'..='\u{9fff}' | '\u{ff00}'..='\u{ffef}'
    )
}
