#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("lecture des réglages impossible")]
    Io(#[from] std::io::Error),
    #[error("écriture des réglages impossible")]
    Write(#[from] lpdnsoft_atomic::AtomicError),
    #[error("sérialisation des réglages impossible")]
    Serialize(#[from] toml::ser::Error),
    #[error("aucune migration depuis la version de schéma {from}")]
    Unsupported { from: u32 },
    #[error(
        "réglages écrits par une version plus récente (schéma {found}, pris en charge : {supported})"
    )]
    ReadOnly { found: u32, supported: u32 },
}
