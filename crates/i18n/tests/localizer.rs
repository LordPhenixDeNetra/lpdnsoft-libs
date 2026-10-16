#![allow(clippy::unwrap_used)]

use lpdnsoft_i18n::{FluentArgs, LanguageIdentifier, Localizer, Resource, langid};

fn localizer() -> Localizer {
    Localizer::new(
        vec![
            Resource {
                locale: langid!("en"),
                source: include_str!("locales/en/demo.ftl"),
            },
            Resource {
                locale: langid!("fr"),
                source: include_str!("locales/fr/demo.ftl"),
            },
        ],
        langid!("en"),
    )
    .unwrap()
}

fn id(s: &str) -> LanguageIdentifier {
    s.parse().unwrap()
}

#[test]
fn translates_in_negotiated_locale() {
    let mut loc = localizer();
    assert_eq!(loc.negotiate(&[langid!("fr")]), langid!("fr"));
    assert_eq!(loc.current(), &langid!("fr"));
    assert_eq!(loc.tr("hello", None), "Bonjour");
}

#[test]
fn negotiates_regional_and_unknown_locales() {
    let mut loc = localizer();
    for (req, want) in [
        ("fr-CA", "fr"),
        ("fr-BE", "fr"),
        ("de-DE", "en"),
        ("und", "en"),
    ] {
        assert_eq!(loc.negotiate(&[id(req)]), id(want), "{req}");
    }
    assert_eq!(loc.negotiate(&[]), langid!("en"));
    assert_eq!(loc.available(), [langid!("en"), langid!("fr")]);
}

#[test]
fn french_plurals() {
    let mut loc = localizer();
    loc.negotiate(&[langid!("fr")]);
    let tr = |n: u32| {
        let mut a = FluentArgs::new();
        a.set("count", n);
        loc.tr("servers-count", Some(&a))
    };
    assert_eq!(tr(1), "1 serveur");
    assert_eq!(tr(12), "12 serveurs");
}

#[test]
fn no_unicode_isolation_marks() {
    let loc = localizer();
    let mut a = FluentArgs::new();
    a.set("count", 3);
    let s = loc.tr("servers-count", Some(&a));
    assert_eq!(s, "3 servers");
    assert!(!s.contains('\u{2068}') && !s.contains('\u{2069}'));
}

#[test]
fn missing_id_returns_id() {
    assert_eq!(localizer().tr("nope", None), "nope");
}

#[test]
fn falls_back_to_english_for_missing_translation() {
    let mut loc = localizer();
    loc.negotiate(&[langid!("fr")]);
    assert_eq!(loc.tr("only-en", None), "Only English");
}

#[test]
fn localizer_is_send_sync() {
    fn assert<T: Send + Sync>() {}
    assert::<Localizer>();
}

#[test]
fn parses_system_locale_strings() {
    use lpdnsoft_i18n::parse_system_locale as p;
    assert_eq!(p("fr_FR.UTF-8"), Some(id("fr-FR")));
    assert_eq!(p("fr-CA"), Some(id("fr-CA")));
    assert_eq!(p("de_DE@euro"), Some(id("de-DE")));
    assert_eq!(p(""), None);
    assert_eq!(p("C"), None);
    assert_eq!(p("C.UTF-8"), None);
    assert_eq!(p("POSIX"), None);
}
