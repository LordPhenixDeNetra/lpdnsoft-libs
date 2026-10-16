//! Écriture de fichier atomique : fichier temporaire dans le même dossier,
//! `fsync`, puis renommage par-dessus la cible.

use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum AtomicError {
    #[error("le chemin {0} n'a pas de dossier parent")]
    NoParent(PathBuf),
    #[error("erreur d'écriture de {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

/// Remplace atomiquement le contenu de `path` par `bytes`.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), AtomicError> {
    let parent = match path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => return Err(AtomicError::NoParent(path.to_path_buf())),
    };
    let io = |source| AtomicError::Io {
        path: path.to_path_buf(),
        source,
    };
    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(io)?;
    tmp.write_all(bytes).map_err(io)?;
    tmp.as_file().sync_all().map_err(io)?;
    tmp.persist(path).map_err(|e| io(e.error))?;
    #[cfg(unix)]
    std::fs::File::open(parent)
        .and_then(|d| d.sync_all())
        .map_err(io)?;
    Ok(())
}
