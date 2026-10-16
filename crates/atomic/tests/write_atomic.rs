use lpdnsoft_atomic::{AtomicError, write_atomic};
use std::fs;

#[test]
fn creates_new_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("f.txt");
    write_atomic(&path, b"hello").unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"hello");
}

#[test]
fn replaces_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("f.txt");
    write_atomic(&path, b"old").unwrap();
    write_atomic(&path, b"new").unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"new");
}

#[test]
fn leaves_no_temp_files() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("f.txt");
    write_atomic(&path, b"hello").unwrap();
    let names: Vec<_> = fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(names, vec!["f.txt"]);
}

#[test]
fn missing_parent_is_io_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("absent").join("f.txt");
    let err = write_atomic(&path, b"x").unwrap_err();
    assert!(matches!(err, AtomicError::Io { .. }), "{err:?}");
    assert!(!dir.path().join("absent").exists());
}

#[test]
fn writes_into_non_ascii_directory() {
    let dir = tempfile::tempdir().unwrap();
    let sub = dir.path().join("Élodie dossier");
    fs::create_dir(&sub).unwrap();
    let path = sub.join("réglages.toml");
    write_atomic(&path, "clé = \"été\"".as_bytes()).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "clé = \"été\"");
}
