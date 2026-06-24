use crate::clean::clean_display;
use crate::clean::display_to_id;
use pkmn_schema::core::web::Name;

/// Converts a display string to a [Name].
///
/// Bare symbol displays keep their display but map to word ids since their symbols are
/// filtered from ids. (ex: the Unown card numbers & ability names `?` -> `question` and
/// `!` -> `exclamation`)
pub fn display_to_name(display: &str) -> Result<Name, String> {
    let display: String = clean_display(display)?;
    match display.as_str() {
        "?" => Ok(Name::new("question".to_string(), display)),
        "!" => Ok(Name::new("exclamation".to_string(), display)),
        _ => Ok(Name::new(display_to_id(&display)?, display)),
    }
}
