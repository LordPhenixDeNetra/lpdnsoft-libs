use lpdnsoft_i18n::{Localizer, Resource, langid, t};

fn main() {
    let loc = Localizer::new(
        vec![Resource { locale: langid!("en"), source: "hello = Hello\n" }],
        langid!("en"),
    )
    .unwrap();
    assert_eq!(t!(&loc, "hello"), "Hello");
    let _ = t!(&loc, "servers-count", count = 2,);
}
