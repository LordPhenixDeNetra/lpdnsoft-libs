//! Traduction Fluent pour les applications LPDNSoft.

mod localizer;
mod system;

pub use fluent_bundle::FluentArgs;
pub use localizer::{I18nError, Localizer, Resource};
pub use lpdnsoft_i18n_catalog::{Catalog, CatalogError, check_complete, load_catalog};
pub use system::{parse_system_locale, system_locales};
pub use unic_langid::{LanguageIdentifier, langid};
