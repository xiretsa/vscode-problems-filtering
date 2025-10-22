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

## Code Coverage

Ce projet est configuré avec `cargo-llvm-cov` pour le code coverage et l'extension VS Code Coverage Gutters pour l'affichage ligne par ligne.

### Génération des rapports

```bash
# Script convenience (recommandé)
./coverage.sh all          # Génère tous les rapports
./coverage.sh summary       # Affiche un résumé
./coverage.sh html          # Rapport HTML seulement
./coverage.sh lcov          # Fichier LCOV pour VS Code

# Commandes cargo directes
cargo llvm-cov --html --output-dir target/coverage
cargo llvm-cov --lcov --output-path target/coverage/lcov.info
cargo llvm-cov --summary-only
```

### Affichage dans VS Code

1. Générez le fichier LCOV : `./coverage.sh lcov`
2. Ouvrez un fichier Rust dans VS Code
3. Utilisez la commande "Coverage Gutters: Display Coverage" (Ctrl+Shift+P)
4. Les lignes couvertes/non-couvertes apparaîtront avec des couleurs dans l'éditeur

### Tâches VS Code

Utilisez `Ctrl+Shift+P` > "Tasks: Run Task" et sélectionnez :
- **Coverage: Complete Report (HTML + LCOV)** - Génère tous les rapports (par défaut)
- **Coverage: Generate HTML Report** - Rapport HTML
- **Coverage: Generate LCOV Report** - Fichier LCOV pour l'extension
- **Coverage: Show Summary** - Résumé dans le terminal

## Bibliothèques utilisées

- `clap` - Parsing des arguments CLI avec derive macros
- `serde` & `serde_json` - Sérialisation/désérialisation JSON
- `anyhow` - Gestion d'erreurs ergonomique
- `tabled` - Affichage en tableau formaté

## Licence

MIT
