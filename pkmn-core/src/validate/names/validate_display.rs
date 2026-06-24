/// Validates the `display` string.
pub fn validate_display(display: &str) -> Result<(), String> {
    if display.is_empty() {
        Err("display names cannot be empty".to_string())
    } else if display.starts_with(' ') || display.ends_with(' ') {
        Err(format!(
            "display names cannot start or end with a space: {}",
            display
        ))
    } else if display.contains("  ") {
        Err(format!("display names contain a double space: {}", display))
    } else if let Some(invalid) = display.chars().find(|c| !is_valid_char(*c)) {
        Err(format!(
            "invalid display name char '{}' in string: {}",
            invalid, display
        ))
    } else {
        Ok(())
    }
}

fn is_valid_char(c: char) -> bool {
    const MALE_FEMALE: &str = "♂♀";
    const ACCENTS: &str = "é";
    const MATH: &str = "−+×";
    const UNITS: &str = "°";
    const ARROWS: &str = "←↑→↓";
    const SHAPES: &str = "☆★◇";

    match c {
        c if c.is_ascii_alphanumeric() => true,
        c if c.is_ascii_punctuation() => true,
        c if MALE_FEMALE.contains(c) => true,
        c if ACCENTS.contains(c) => true,
        c if MATH.contains(c) => true,
        c if UNITS.contains(c) => true,
        c if ARROWS.contains(c) => true,
        c if SHAPES.contains(c) => true,
        c if is_greek(c) => true,
        c if is_japanese(c) => true,
        ' ' => true,
        _ => false,
    }
}

fn is_greek(c: char) -> bool {
    matches!(
        c,
        '\u{0391}'..='\u{03A9}'  // capital alpha .. omega
        | '\u{03B1}'..='\u{03C9}' // small alpha .. omega (incl. final sigma)
    )
}

fn is_japanese(c: char) -> bool {
    matches!(
        c,
        '\u{3000}'..='\u{303F}'    // CJK Symbols and Punctuation (「」、。)
        | '\u{3040}'..='\u{309F}'  // Hiragana
        | '\u{30A0}'..='\u{30FF}'  // Katakana
        | '\u{31F0}'..='\u{31FF}'  // Katakana Phonetic Extensions
        | '\u{3400}'..='\u{4DBF}'  // CJK Unified Ideographs Extension A
        | '\u{4E00}'..='\u{9FFF}'  // CJK Unified Ideographs (Kanji)
        | '\u{FF00}'..='\u{FFEF}'  // Halfwidth and Fullwidth Forms
    )
}
