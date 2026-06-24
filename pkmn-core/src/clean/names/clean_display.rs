use crate::validate::validate_display;

/// Cleans the `display` string.
pub fn clean_display(display: &str) -> Result<String, String> {
    let s: String = reconstruct(display);
    validate_display(&s)?;
    Ok(s.to_string())
}

/// Reconstructs the string `s`.
///
/// - Removes zero-width spaces.
/// - Converts ASCII control chars to spaces.
/// - Removes instances of double spaces.
/// - Maps weird characters to more normal characters for SEO.
fn reconstruct(s: &str) -> String {
    let s: &str = s.trim();
    let mut r: String = String::with_capacity(s.len());
    let mut last_space: bool = false;
    for c in s.chars() {
        let c: char = if c.is_ascii_control() { ' ' } else { c };

        // zero-width space
        if c == '\u{feff}' {
            continue;
        }

        if c == ' ' {
            if !last_space {
                r.push(' ');
                last_space = true;
            }
        } else {
            let c: char = match c {
                '’' | '‘' => '\'',
                '“' | '”' => '"',
                '—' | '–' => '-',
                'º' => '°',
                '⇢' => '→',
                '⍺' | '𝛼' => 'α',
                '𝛽' => 'β',
                '𝛾' => 'γ',
                '𝛿' => 'δ',
                c => c,
            };
            let mut buf: [u8; 4] = [0; 4];
            let c: &str = match c {
                '…' => "...",
                c => c.encode_utf8(&mut buf),
            };
            r.push_str(c);
            last_space = false;
        }
    }
    r
}
