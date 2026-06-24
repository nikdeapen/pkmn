/// Validates the `id` string.
pub fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        Err("ids cannot be empty".to_string())
    } else if id.starts_with('-') || id.ends_with(' ') {
        Err(format!("ids cannot start or end with a dash: {}", id))
    } else if id.contains("---") {
        Err(format!(
            "ids cannot contain three sequential dashes (---): {}",
            id
        ))
    } else if let Some(invalid) = id.chars().find(|c| !is_valid_char(*c)) {
        Err(format!("invalid id char '{}' in string:{}", invalid, id))
    } else {
        Ok(())
    }
}

fn is_valid_char(c: char) -> bool {
    match c {
        c if c.is_ascii_digit() => true,
        c if c.is_ascii_lowercase() => true,
        '-' => true,
        _ => false,
    }
}
