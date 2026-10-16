# lpdnsoft-libs

Bibliothèques Rust communes aux applications de la suite **LPDNSoft** (dont aNkou).

| Crate | Rôle |
|---|---|
| `lpdnsoft-atomic` | Écriture de fichier atomique |
| `lpdnsoft-log` | Journalisation `tracing` (console + fichiers rotatifs, masquage des secrets) |
| `lpdnsoft-settings` | Réglages TOML versionnés, migrations, récupération |
| `lpdnsoft-i18n` | Traduction Fluent, négociation de langue, macro `t!` vérifiée à la compilation |

Contrôles : `cargo xtask ci`.

Licence : MIT ou Apache-2.0, au choix.
