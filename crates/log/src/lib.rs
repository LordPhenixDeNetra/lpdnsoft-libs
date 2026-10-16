//! Initialisation `tracing` commune : console (stderr) + fichiers journaliers rotatifs.

mod redact;

pub use redact::Redacted;

use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{Builder, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, fmt};

/// Nombre de fichiers journaliers conservés.
const MAX_LOG_FILES: usize = 7;

pub struct LogConfig {
    pub app_name: &'static str,
    /// Dossier des fichiers de log ; `None` = console seule.
    pub dir: Option<PathBuf>,
    /// Filtre utilisé si `RUST_LOG` est absent ou invalide.
    pub default_filter: &'static str,
}

/// À garder en vie jusqu'à la fin du programme : sa destruction vide le tampon fichier.
pub struct LogGuard {
    pub file_logging: bool,
    fallback_reason: Option<String>,
    _worker: Option<WorkerGuard>,
}

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("un subscriber tracing global est déjà installé")]
    AlreadyInitialized,
}

/// Construit le subscriber sans l'installer globalement.
pub fn build(cfg: &LogConfig) -> (Box<dyn tracing::Subscriber + Send + Sync>, LogGuard) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(cfg.default_filter));

    let mut fallback_reason = None;
    let appender = cfg.dir.as_ref().and_then(|dir| {
        Builder::new()
            .rotation(Rotation::DAILY)
            .filename_prefix(cfg.app_name)
            .filename_suffix("log")
            .max_log_files(MAX_LOG_FILES)
            .build(dir)
            .map_err(|e| fallback_reason = Some(format!("{}: {e}", dir.display())))
            .ok()
    });
    let (file_layer, worker) = match appender {
        Some(a) => {
            let (writer, worker) = tracing_appender::non_blocking(a);
            (
                Some(fmt::layer().with_ansi(false).with_writer(writer)),
                Some(worker),
            )
        }
        None => (None, None),
    };

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(file_layer);
    let guard = LogGuard {
        file_logging: worker.is_some(),
        fallback_reason,
        _worker: worker,
    };
    (Box::new(subscriber), guard)
}

/// Installe le subscriber global. Si le dossier de logs est inutilisable,
/// continue en console seule et le signale par un `warn!`.
pub fn init(cfg: LogConfig) -> Result<LogGuard, LogError> {
    let (subscriber, guard) = build(&cfg);
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| LogError::AlreadyInitialized)?;
    if let Some(reason) = &guard.fallback_reason {
        tracing::warn!(%reason, "journalisation fichier indisponible, console seule");
    }
    Ok(guard)
}
