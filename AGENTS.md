# lpdnsoft-libs — consignes pour les agents

Bibliothèques Rust communes aux applications LPDNSoft (aNkou…). Aucune dépendance vers une app.

## Structure

- `crates/atomic` — `write_atomic` : fichier temporaire + fsync + rename.
- `crates/log` — `init` / `build` : `tracing` console + fichiers journaliers (7 conservés) ; `Redacted` masque les secrets.
- `crates/settings` — `SettingsStore` : réglages TOML versionnés (`schema_version`), migrations, sauvegarde des fichiers corrompus, lecture seule si le schéma est plus récent.
- `crates/i18n-catalog` — lecture/validation des `.ftl` (sans proc-macro).
- `crates/i18n-macros` — macro `t!` vérifiée à la compilation.
- `crates/i18n` — `Localizer`, négociation de langue, `system_locales`, ré-exporte `t!`.
- `xtask` — contrôles.

## Commande unique

```sh
cargo xtask ci   # fmt --check, clippy -D warnings, tests, cargo deny check
```

Prérequis : Rust stable ≥ 1.90, `cargo install cargo-deny --locked`.

## Règles

- Édition 2024, `resolver = "3"`, licence `MIT OR Apache-2.0`.
- `unsafe_code = "forbid"` ; clippy `unwrap_used`, `expect_used`, `dbg_macro`, `todo` en `deny` (autorisés dans les tests).
- Une erreur `thiserror` par crate ; `anyhow` seulement dans `xtask` et les binaires d'app.
- Préfixe des crates : `lpdnsoft-`. Dossiers système : `ProjectDirs::from("com", "LPDNSoft", <app>)`.
- Langues : `en` et `fr`, repli `en`.

## i18n

Toute chaîne visible passe par `t!(&loc, "id", arg = valeur)`. Ajouter chaque clé en `en` **et** `fr` : une clé absente d'une langue casse la compilation. La macro lit `LPDNSOFT_I18N_LOCALES_DIR` (sinon `$CARGO_MANIFEST_DIR/locales`) ; ici, `.cargo/config.toml` le fait pointer vers `crates/i18n/tests/locales`.

## Commits

Conventional Commits en français (`feat(settings): …`, `fix(i18n): …`, `ci: …`, `docs: …`), terminés par la ligne `Co-Authored-By:` de l'agent le cas échéant. Windows : `git config core.longpaths true`, dossier parent court (`C:\dev\LPDNSoft`).
