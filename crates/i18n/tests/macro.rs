#![allow(clippy::unwrap_used)]

use lpdnsoft_i18n::{Localizer, Resource, langid, t};

fn french() -> Localizer {
    let mut loc = Localizer::new(
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
    .unwrap();
    loc.negotiate(&[langid!("fr")]);
    loc
}

#[test]
fn t_macro_translates() {
    let loc = french();
    assert_eq!(t!(&loc, "hello"), "Bonjour");
}

#[test]
fn t_macro_passes_args() {
    let loc = french();
    assert_eq!(t!(&loc, "servers-count", count = 12), "12 serveurs");
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/ok.rs");
    t.compile_fail("tests/ui/missing_key.rs");
}
