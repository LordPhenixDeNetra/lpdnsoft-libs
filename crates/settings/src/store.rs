use crate::SettingsError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION_KEY: &str = "schema_version";

pub trait Settings: Serialize + DeserializeOwned + Default {
    /// Nom du fichier dans le dossier du store, ex. `"settings.toml"`.
    const FILE_NAME: &'static str;
    /// Version courante du schéma (≥ 1).
    const SCHEMA_VERSION: u32;

    /// Migre une table brute de `from` vers `from + 1`.
    fn migrate_step(from: u32, table: toml::Table) -> Result<toml::Table, SettingsError> {
        let _ = table;
        Err(SettingsError::Unsupported { from })
    }
}

#[derive(Debug)]
pub struct Loaded<S> {
    pub settings: S,
    pub recovery: Option<Recovery>,
    /// Fichier écrit par une version plus récente : `save` le refusera.
    pub read_only: bool,
}

#[derive(Debug)]
pub enum Recovery {
    /// Fichier illisible : copié vers `backup`, valeurs par défaut utilisées.
    Corrupt { backup: PathBuf },
    /// Migré en mémoire depuis `from` ; rien n'est écrit tant que `save` n'est pas appelé.
    Migrated { from: u32 },
}

pub struct SettingsStore<S: Settings> {
    dir: PathBuf,
    _marker: PhantomData<S>,
}

enum Parsed {
    Table { version: u32, table: toml::Table },
    Corrupt,
}

impl<S: Settings> SettingsStore<S> {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            _marker: PhantomData,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.dir.join(S::FILE_NAME)
    }

    pub fn load(&self) -> Result<Loaded<S>, SettingsError> {
        let path = self.path();
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::loaded(S::default(), None, false));
            }
            Err(e) => return Err(e.into()),
        };
        let (version, mut table) = match parse(&bytes) {
            Parsed::Table { version, table } => (version, table),
            Parsed::Corrupt => return self.recover_corrupt(),
        };
        if version > S::SCHEMA_VERSION {
            return Ok(Self::loaded(S::default(), None, true));
        }
        for from in version..S::SCHEMA_VERSION {
            table = S::migrate_step(from, table)?;
        }
        match S::deserialize(toml::Value::Table(table)) {
            Ok(settings) => {
                let recovery =
                    (version < S::SCHEMA_VERSION).then_some(Recovery::Migrated { from: version });
                Ok(Self::loaded(settings, recovery, false))
            }
            Err(_) => self.recover_corrupt(),
        }
    }

    pub fn save(&self, loaded: &Loaded<S>) -> Result<(), SettingsError> {
        let on_disk = self.disk_version();
        if loaded.read_only || on_disk.is_some_and(|v| v > S::SCHEMA_VERSION) {
            return Err(SettingsError::ReadOnly {
                found: on_disk.unwrap_or(0),
                supported: S::SCHEMA_VERSION,
            });
        }
        // `schema_version` écrit en tête, à la main : `toml::Table` trie ses clés.
        let body = toml::to_string(&loaded.settings)?;
        let text = format!("{VERSION_KEY} = {}\n{body}", S::SCHEMA_VERSION);
        lpdnsoft_atomic::write_atomic(&self.path(), text.as_bytes())?;
        Ok(())
    }

    fn loaded(settings: S, recovery: Option<Recovery>, read_only: bool) -> Loaded<S> {
        Loaded {
            settings,
            recovery,
            read_only,
        }
    }

    fn recover_corrupt(&self) -> Result<Loaded<S>, SettingsError> {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let backup = self.dir.join(format!("{}.corrupt-{secs}", S::FILE_NAME));
        std::fs::copy(self.path(), &backup)?;
        Ok(Self::loaded(
            S::default(),
            Some(Recovery::Corrupt { backup }),
            false,
        ))
    }

    fn disk_version(&self) -> Option<u32> {
        match parse(&std::fs::read(self.path()).ok()?) {
            Parsed::Table { version, .. } => Some(version),
            Parsed::Corrupt => None,
        }
    }
}

/// Table TOML sans la clé `schema_version` (absente = 1).
fn parse(bytes: &[u8]) -> Parsed {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return Parsed::Corrupt;
    };
    let Ok(mut table) = text.parse::<toml::Table>() else {
        return Parsed::Corrupt;
    };
    let version = match table.remove(VERSION_KEY) {
        None => 1,
        Some(toml::Value::Integer(v)) => match u32::try_from(v) {
            Ok(v) if v >= 1 => v,
            _ => return Parsed::Corrupt,
        },
        Some(_) => return Parsed::Corrupt,
    };
    Parsed::Table { version, table }
}
