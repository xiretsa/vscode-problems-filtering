# VS Code Problems Filtering

Une application CLI Rust pour filtrer les problèmes exportés depuis la vue Problèmes de VS Code.

## Fonctionnalités

- Filtrage par termes d'inclusion (tous doivent être présents)
- Filtrage par termes d'exclusion (aucun ne doit être présent)
- Support des comparaisons insensibles à la casse
- Affichage en tableau formaté
- Gestion d'erreurs robuste

## Installation

```bash
cargo build --release
```

## Utilisation

```bash
# Afficher l'aide
cargo run -- --help

# Filtrer les problèmes contenant "deprecated"
cargo run -- -f deprecated.json -i "deprecated"

# Filtrer les problèmes contenant "deprecated" mais excluant "ActionError"
cargo run -- -f deprecated.json -i "deprecated" -e "ActionError"

# Filtrage insensible à la casse
cargo run -- -f deprecated.json -i "DEPRECATED" --ignore-case

# Plusieurs termes d'inclusion et d'exclusion
cargo run -- -f deprecated.json -i "deprecated" -i "warning" -e "test" -e "mock"

# Compter seulement les résultats (sans afficher le tableau)
cargo run -- -f deprecated.json -i "deprecated" --count-only

# Sortie au format JSON
cargo run -- -f deprecated.json -i "constructor" -e "sonarqube" --json
```

## Options

- `-f, --input <FILE>`: Fichier JSON d'entrée (requis)
- `-i, --include <TERM>`: Terme à inclure (répétable)
- `-e, --exclude <TERM>`: Terme à exclure (répétable)
- `--ignore-case`: Ignorer la casse lors des comparaisons
- `-c, --count-only`: Afficher seulement le nombre de résultats
- `--json`: Sortie au format JSON

## Format du fichier d'entrée

Le fichier JSON doit contenir un tableau d'objets représentant les problèmes VS Code, avec au minimum les champs :
- `resource`: le chemin du fichier
- `message`: le message du problème
- `startLineNumber`: le numéro de ligne

## Tests

```bash
cargo test
```

## Bibliothèques utilisées

- `clap` - Parsing des arguments CLI avec derive macros
- `serde` & `serde_json` - Sérialisation/désérialisation JSON
- `anyhow` - Gestion d'erreurs ergonomique
- `tabled` - Affichage en tableau formaté

## Licence

MIT
