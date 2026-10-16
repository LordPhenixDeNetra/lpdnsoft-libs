use lpdnsoft_i18n::{Localizer, Resource, langid, t};

fn main() {
    let loc = Localizer::new(
        vec![Resource { locale: langid!("en"), source: "hello = Hello\n" }],
        langid!("en"),
    )
    .unwrap();
    let _ = t!(&loc, "does-not-exist");
}
