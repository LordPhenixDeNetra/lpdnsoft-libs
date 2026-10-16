use unic_langid::LanguageIdentifier;

/// Locales préférées du système, dans l'ordre ; les entrées invalides sont ignorées.
pub fn system_locales() -> Vec<LanguageIdentifier> {
    sys_locale::get_locales()
        .filter_map(|s| parse_system_locale(&s))
        .collect()
}

/// Convertit une locale système (`fr_FR.UTF-8`, `de_DE@euro`, `fr-CA`) ;
/// `C`, `POSIX` et les chaînes vides ou invalides donnent `None`.
pub fn parse_system_locale(raw: &str) -> Option<LanguageIdentifier> {
    let base = raw.split(['.', '@']).next().unwrap_or("").replace('_', "-");
    if base.is_empty() || base.eq_ignore_ascii_case("c") || base.eq_ignore_ascii_case("posix") {
        return None;
    }
    base.parse().ok()
}
