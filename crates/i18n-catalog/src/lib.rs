//! Lecture et validation des catalogues Fluent : `<dir>/<locale>/*.ftl`.
//!
//! Sans proc-macro, pour servir à la fois la macro `t!`, les tests et `xtask`.

use fluent_syntax::ast::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// locale → identifiants des messages (les termes gardent leur préfixe `-`).
#[derive(Debug, Default)]
pub struct Catalog {
    pub locales: BTreeMap<String, BTreeSet<String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("lecture de {path} impossible : {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("erreurs de syntaxe Fluent dans {file} : {errors:?}")]
    Parse { file: PathBuf, errors: Vec<String> },
    #[error("messages manquants (locale, id) : {missing:?}")]
    Incomplete { missing: Vec<(String, String)> },
    #[error("aucune locale trouvée")]
    Empty,
}

/// Charge tous les `*.ftl` de chaque sous-dossier (= locale) de `locales_dir`.
pub fn load_catalog(locales_dir: &Path) -> Result<Catalog, CatalogError> {
    let mut catalog = Catalog::default();
    for locale_dir in sorted_entries(locales_dir)? {
        if !locale_dir.is_dir() {
            continue;
        }
        let Some(locale) = locale_dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
        else {
            continue;
        };
        let ids = catalog.locales.entry(locale).or_default();
        for file in sorted_entries(&locale_dir)? {
            if file.extension().is_some_and(|e| e == "ftl") {
                ids.extend(message_ids(&file)?);
            }
        }
    }
    if catalog.locales.is_empty() {
        return Err(CatalogError::Empty);
    }
    Ok(catalog)
}

/// Vérifie que chaque id existe dans toutes les locales.
pub fn check_complete(c: &Catalog) -> Result<(), CatalogError> {
    let all: BTreeSet<&String> = c.locales.values().flatten().collect();
    let missing: Vec<(String, String)> = c
        .locales
        .iter()
        .flat_map(|(locale, ids)| {
            all.iter()
                .filter(|id| !ids.contains(**id))
                .map(|id| (locale.clone(), (*id).clone()))
        })
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(CatalogError::Incomplete { missing })
    }
}

/// Locales (triées) où `id` est absent.
pub fn missing_locales_for(c: &Catalog, id: &str) -> Vec<String> {
    c.locales
        .iter()
        .filter(|(_, ids)| !ids.contains(id))
        .map(|(locale, _)| locale.clone())
        .collect()
}

fn sorted_entries(dir: &Path) -> Result<Vec<PathBuf>, CatalogError> {
    let io = |source| CatalogError::Io {
        path: dir.to_path_buf(),
        source,
    };
    let mut paths = std::fs::read_dir(dir)
        .map_err(io)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(io)?;
    paths.sort();
    Ok(paths)
}

fn message_ids(file: &Path) -> Result<Vec<String>, CatalogError> {
    let source = std::fs::read_to_string(file).map_err(|source| CatalogError::Io {
        path: file.to_path_buf(),
        source,
    })?;
    let resource = fluent_syntax::parser::parse(source.as_str()).map_err(|(_, errors)| {
        CatalogError::Parse {
            file: file.to_path_buf(),
            errors: errors.iter().map(|e| format!("{e:?}")).collect(),
        }
    })?;
    Ok(resource
        .body
        .iter()
        .filter_map(|entry| match entry {
            Entry::Message(m) => Some(m.id.name.to_string()),
            Entry::Term(t) => Some(format!("-{}", t.id.name)),
            _ => None,
        })
        .collect())
}
