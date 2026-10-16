#![allow(clippy::unwrap_used)]

use lpdnsoft_settings::{Recovery, Settings, SettingsError, SettingsStore};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
struct Demo {
    language: String,
    kill_switch: bool,
}

impl Settings for Demo {
    const FILE_NAME: &'static str = "settings.toml";
    const SCHEMA_VERSION: u32 = 2;

    fn migrate_step(from: u32, mut table: toml::Table) -> Result<toml::Table, SettingsError> {
        match from {
            1 => {
                if let Some(v) = table.remove("lang") {
                    table.insert("language".into(), v);
                }
                Ok(table)
            }
            _ => Err(SettingsError::Unsupported { from }),
        }
    }
}

fn store() -> (tempfile::TempDir, SettingsStore<Demo>) {
    let dir = tempfile::tempdir().unwrap();
    let store = SettingsStore::new(dir.path().to_path_buf());
    (dir, store)
}

#[test]
fn missing_file_returns_defaults() {
    let (_dir, store) = store();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.settings, Demo::default());
    assert!(loaded.recovery.is_none());
    assert!(!loaded.read_only);
    assert!(!store.path().exists());
}

#[test]
fn save_then_load_roundtrip() {
    let (_dir, store) = store();
    let mut loaded = store.load().unwrap();
    loaded.settings = Demo {
        language: "fr".into(),
        kill_switch: true,
    };
    store.save(&loaded).unwrap();
    let text = fs::read_to_string(store.path()).unwrap();
    assert!(text.starts_with("schema_version = 2"), "{text}");
    let again = store.load().unwrap();
    assert_eq!(again.settings, loaded.settings);
    assert!(again.recovery.is_none());
}

#[test]
fn corrupt_file_is_backed_up_and_defaults_returned() {
    let (dir, store) = store();
    fs::write(store.path(), "kill_switch = 'oui").unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.settings, Demo::default());
    let Some(Recovery::Corrupt { backup }) = loaded.recovery else {
        panic!("attendu Corrupt, obtenu {:?}", loaded.recovery);
    };
    assert_eq!(backup.parent().unwrap(), dir.path());
    let name = backup.file_name().unwrap().to_string_lossy().into_owned();
    let secs = name.strip_prefix("settings.toml.corrupt-").unwrap();
    assert!(secs.parse::<u64>().is_ok(), "{name}");
    assert_eq!(fs::read_to_string(&backup).unwrap(), "kill_switch = 'oui");
}

#[test]
fn wrong_types_are_treated_as_corrupt() {
    let (_dir, store) = store();
    fs::write(
        store.path(),
        "schema_version = 2\nlanguage = \"fr\"\nkill_switch = \"oui\"\n",
    )
    .unwrap();
    let loaded = store.load().unwrap();
    assert!(matches!(loaded.recovery, Some(Recovery::Corrupt { .. })));
    assert_eq!(loaded.settings, Demo::default());
}

#[test]
fn older_schema_is_migrated() {
    let (_dir, store) = store();
    fs::write(
        store.path(),
        "schema_version = 1\nlang = \"fr\"\nkill_switch = true\n",
    )
    .unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.settings.language, "fr");
    assert!(matches!(
        loaded.recovery,
        Some(Recovery::Migrated { from: 1 })
    ));
}

#[test]
fn newer_schema_is_read_only() {
    let (_dir, store) = store();
    let original = "schema_version = 99\nlanguage = \"fr\"\nfutur = 1\n";
    fs::write(store.path(), original).unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.settings, Demo::default());
    assert!(loaded.read_only);
    let err = store.save(&loaded).unwrap_err();
    assert!(
        matches!(
            err,
            SettingsError::ReadOnly {
                found: 99,
                supported: 2
            }
        ),
        "{err:?}"
    );
    assert_eq!(fs::read_to_string(store.path()).unwrap(), original);
}

#[test]
fn missing_schema_version_is_treated_as_1() {
    let (_dir, store) = store();
    fs::write(store.path(), "lang = \"en\"\nkill_switch = false\n").unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.settings.language, "en");
    assert!(matches!(
        loaded.recovery,
        Some(Recovery::Migrated { from: 1 })
    ));
}
