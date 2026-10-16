//! Réglages TOML typés et versionnés, écrits atomiquement.
//!
//! Format disque : `schema_version = N` puis les champs du type au même niveau.

mod error;
mod paths;
mod store;

pub use error::SettingsError;
pub use paths::config_dir;
pub use store::{Loaded, Recovery, Settings, SettingsStore};
