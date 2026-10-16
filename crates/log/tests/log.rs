//! Les helpers hors `#[test]` ne sont pas couverts par `allow-unwrap-in-tests`.
#![allow(clippy::unwrap_used)]

use lpdnsoft_log::{LogConfig, Redacted, build};
use std::fs;
use std::path::Path;

fn cfg(dir: &Path) -> LogConfig {
    LogConfig {
        app_name: "demo",
        dir: Some(dir.to_path_buf()),
        default_filter: "info",
    }
}

fn log_contents(dir: &Path) -> String {
    let files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy().into_owned();
            n.starts_with("demo.") && n.ends_with(".log")
        })
        .collect();
    assert_eq!(files.len(), 1, "{files:?}");
    fs::read_to_string(&files[0]).unwrap()
}

#[test]
fn writes_to_daily_file() {
    let dir = tempfile::tempdir().unwrap();
    let (sub, guard) = build(&cfg(dir.path()));
    assert!(guard.file_logging);
    tracing::subscriber::with_default(sub, || tracing::info!("bonjour"));
    drop(guard);
    assert!(log_contents(dir.path()).contains("bonjour"));
}

#[test]
fn unwritable_dir_falls_back_to_console() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("pas-un-dossier");
    fs::write(&file, "x").unwrap();
    let (sub, guard) = build(&cfg(&file));
    assert!(!guard.file_logging);
    tracing::subscriber::with_default(sub, || tracing::info!("toujours vivant"));
}

#[test]
fn redacted_hides_value() {
    assert_eq!(format!("{}", Redacted("secret")), "[redacted]");
    assert_eq!(format!("{:?}", Redacted("secret")), "[redacted]");
}

#[test]
fn redacted_value_not_in_log_file() {
    let dir = tempfile::tempdir().unwrap();
    let (sub, guard) = build(&cfg(dir.path()));
    tracing::subscriber::with_default(
        sub,
        || tracing::info!(token = %Redacted("abc123"), "connexion"),
    );
    drop(guard);
    let text = log_contents(dir.path());
    assert!(
        text.contains("connexion") && text.contains("[redacted]"),
        "{text}"
    );
    assert!(!text.contains("abc123"));
}
