use pkmn_schema::cards::action::Damage;
use pkmn_schema::cards::meta::DamageModifier;

/// Cleans an attack `damage` value. (ex: `30`, `30+`, `30×`, `?`)
pub fn clean_damage(damage: &str) -> Result<Damage, String> {
    let damage: &str = damage.trim();
    if damage == "?" {
        return Ok(Damage::default().with_variable(true));
    }
    let (scalar, modifier): (&str, Option<DamageModifier>) =
        if let Some(scalar) = damage.strip_suffix('+') {
            (scalar, Some(DamageModifier::Plus))
        } else if let Some(scalar) = damage.strip_suffix('×') {
            (scalar, Some(DamageModifier::Times))
        } else if let Some(scalar) = damage.strip_suffix(['-', '−']) {
            (scalar, Some(DamageModifier::Minus))
        } else {
            (damage, None)
        };
    let scalar: u32 = scalar
        .trim()
        .parse()
        .map_err(|e| format!("invalid damage '{damage}': {e}"))?;
    let mut result: Damage = Damage::default().with_scalar(scalar);
    if let Some(modifier) = modifier {
        result.set_modifier(modifier);
    }
    Ok(result)
}

/// Cleans a weakness/resistance modifier `value` into its scalar & [DamageModifier].
/// (ex: `×2` -> `(2, Times)`, `−30` -> `(30, Minus)`)
pub fn clean_modifier_value(value: &str) -> Result<(u32, DamageModifier), String> {
    let value: &str = value.trim();
    let (modifier, scalar): (DamageModifier, &str) = if let Some(scalar) = value.strip_prefix('×')
    {
        (DamageModifier::Times, scalar)
    } else if let Some(scalar) = value.strip_prefix(['-', '−']) {
        (DamageModifier::Minus, scalar)
    } else if let Some(scalar) = value.strip_prefix('+') {
        (DamageModifier::Plus, scalar)
    } else {
        return Err(format!("invalid modifier value: {value}"));
    };
    let scalar: u32 = scalar
        .trim()
        .parse()
        .map_err(|e| format!("invalid modifier value '{value}': {e}"))?;
    Ok((scalar, modifier))
}
