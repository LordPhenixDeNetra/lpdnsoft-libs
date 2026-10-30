# Contribuer à lpdnsoft-libs

Bibliothèques partagées de la suite **LPDNSoft** : écriture atomique de fichiers, i18n Fluent,
journalisation, réglages, et le thème egui commun. Elles sont utilisées par `ankou` et par
`lpdnsoft-launcher`, par dépendances de chemin.

## Prérequis

- Rust stable **≥ 1.90** (édition 2024) — **1.95** pour `lpdnsoft-ui`, qui suit egui 0.36 ;
- `cargo install cargo-deny --locked` ;
- les deux consommateurs clonés **à côté** de ce dépôt, pour vérifier qu'ils compilent
  toujours :

```
N:\dev\LPDNSoft\lpdnsoft-libs
N:\dev\LPDNSoft\ankou
N:\dev\LPDNSoft\lpdnsoft-launcher
```

## Construire et tester

```sh
cargo xtask ci        # fmt, clippy, tests, deny
cargo test -p lpdnsoft-atomic
```

## Règles

- **Ce sont des bibliothèques** : une API publique est un contrat. La modifier sans raison est
  une régression pour deux applications, donc un changement de version.
- **`unsafe` est interdit.** Clippy refuse `unwrap`, `expect`, `dbg!` et `todo!` hors tests.
- **Une erreur `thiserror` par crate** ; jamais `anyhow` ici.
- **Le minimum supporté est 1.90** : une fonctionnalité plus récente casserait les
  consommateurs. La CI vérifie ce minimum.
- **Documenter ce qui est public** : un item exporté sans doc est une question laissée au
  lecteur.

## Tests

- **Logique privée** : `#[cfg(test)] mod tests` dans le fichier. **API publique** :
  `crates/<crate>/tests/*.rs`.
- **Un test nouveau échoue sans le changement.**
- Un test qui touche le disque utilise `tempfile`, jamais un chemin fixe.

## Commits

- **Conventional Commits**, en anglais, avec portée : `feat(atomic):`, `fix(i18n):`,
  `docs(ui):`.
- **Un commit par sujet**, `git add` avec des chemins explicites, `git commit -s` (DCO).
- **Un `-m` par paragraphe** : le titre, puis le pourquoi.

## Branches et demande de fusion

- Branche `feat/…`, `fix/…` ou `docs/…`, fusion vers `main`.
- Titre au format des commits ; description selon `.github/pull_request_template.md`.
- Une PR qui change l'API publique dit **quels consommateurs** ont été vérifiés.
