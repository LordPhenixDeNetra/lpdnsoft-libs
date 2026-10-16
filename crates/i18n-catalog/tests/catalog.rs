use lpdnsoft_i18n_catalog::{CatalogError, check_complete, load_catalog, missing_locales_for};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn ok_fixture_is_complete() {
    let c = load_catalog(&fixture("ok")).unwrap();
    assert_eq!(c.locales.keys().collect::<Vec<_>>(), ["en", "fr"]);
    check_complete(&c).unwrap();
    assert!(missing_locales_for(&c, "hello").is_empty());
}

#[test]
fn missing_key_is_reported() {
    let c = load_catalog(&fixture("missing")).unwrap();
    let err = check_complete(&c).unwrap_err();
    let CatalogError::Incomplete { missing } = err else {
        panic!("attendu Incomplete, obtenu {err:?}");
    };
    assert_eq!(missing, [("fr".to_string(), "servers-count".to_string())]);
    assert_eq!(missing_locales_for(&c, "servers-count"), ["fr"]);
    assert_eq!(missing_locales_for(&c, "nope"), ["en", "fr"]);
}

#[test]
fn parse_error_names_the_file() {
    let err = load_catalog(&fixture("broken")).unwrap_err();
    let CatalogError::Parse { file, errors } = err else {
        panic!("attendu Parse, obtenu {err:?}");
    };
    assert!(file.to_string_lossy().contains("broken"), "{file:?}");
    assert!(!errors.is_empty());
}

#[test]
fn empty_dir_is_error() {
    let err = load_catalog(&fixture("empty")).unwrap_err();
    assert!(matches!(err, CatalogError::Empty), "{err:?}");
}
