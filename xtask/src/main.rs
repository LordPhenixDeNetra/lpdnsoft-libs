//! `cargo xtask ci` : les mêmes contrôles en local et en CI.

use anyhow::{Context, Result, bail};
use std::process::Command;

const STEPS: &[&[&str]] = &[
    &["fmt", "--all", "--check"],
    &[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ],
    &["test", "--workspace"],
    &["deny", "check"],
];

fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("ci") => ci(),
        _ => bail!("usage : cargo xtask ci"),
    }
}

fn ci() -> Result<()> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let deny_ok = Command::new(&cargo)
        .args(["deny", "--version"])
        .output()
        .is_ok_and(|o| o.status.success());
    if !deny_ok {
        bail!("cargo-deny manquant : cargo install cargo-deny --locked");
    }
    for args in STEPS {
        let shown = format!("cargo {}", args.join(" "));
        eprintln!("==> {shown}");
        let status = Command::new(&cargo)
            .args(*args)
            .status()
            .with_context(|| format!("lancement de `{shown}` impossible"))?;
        if !status.success() {
            bail!("`{shown}` a échoué ({status})");
        }
    }
    eprintln!("==> ci OK");
    Ok(())
}
