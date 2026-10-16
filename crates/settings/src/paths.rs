use std::path::PathBuf;

/// Dossier de configuration de l'application `app` selon l'OS.
pub fn config_dir(app: &str) -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "LPDNSoft", app).map(|d| d.config_dir().to_path_buf())
}
