#!/bin/bash

# Script pour la gestion du code coverage
# Utilisation: ./coverage.sh [command]
# Commandes disponibles:
#   html     - Génère un rapport HTML
#   lcov     - Génère un fichier LCOV pour VS Code
#   json     - Génère un rapport JSON
#   summary  - Affiche un résumé
#   all      - Génère tous les rapports
#   open     - Ouvre le rapport HTML dans le navigateur
#   clean    - Nettoie les rapports existants

set -e

COVERAGE_DIR="target/coverage"

case "$1" in
    "html")
        echo "🔍 Génération du rapport HTML..."
        cargo llvm-cov --html --output-dir "$COVERAGE_DIR"
        echo "✅ Rapport HTML généré dans $COVERAGE_DIR/html/index.html"
        ;;
    "lcov")
        echo "🔍 Génération du fichier LCOV..."
        mkdir -p ${COVERAGE_DIR}
        cargo llvm-cov --lcov --output-path "$COVERAGE_DIR/lcov.info"
        echo "✅ Fichier LCOV généré dans $COVERAGE_DIR/lcov.info"
        ;;
    "json")
        echo "🔍 Génération du rapport JSON..."
        mkdir -p ${COVERAGE_DIR}
        cargo llvm-cov --json --output-path "$COVERAGE_DIR/coverage.json"
        echo "✅ Rapport JSON généré dans $COVERAGE_DIR/coverage.json"
        ;;
    "summary")
        echo "🔍 Résumé de la couverture:"
        cargo llvm-cov --summary-only
        ;;
    "all")
        echo "🔍 Génération de tous les rapports..."
        mkdir -p "$COVERAGE_DIR"
        cargo llvm-cov --html --output-dir "$COVERAGE_DIR"
        cargo llvm-cov --lcov --output-path "$COVERAGE_DIR/lcov.info"
        cargo llvm-cov --json --output-path "$COVERAGE_DIR/coverage.json"
        echo "✅ Tous les rapports générés dans $COVERAGE_DIR/"
        ;;
    "open")
        if [ -f "$COVERAGE_DIR/html/index.html" ]; then
            echo "🌐 Ouverture du rapport HTML..."
            # Dans un dev container, utilise la variable d'environnement BROWSER
            "$BROWSER" "$COVERAGE_DIR/html/index.html" 2>/dev/null || \
            xdg-open "$COVERAGE_DIR/html/index.html" 2>/dev/null || \
            echo "❌ Impossible d'ouvrir le rapport. Ouvrez manuellement: $COVERAGE_DIR/html/index.html"
        else
            echo "❌ Rapport HTML non trouvé. Exécutez d'abord: ./coverage.sh html"
        fi
        ;;
    "clean")
        echo "🧹 Nettoyage des rapports existants..."
        rm -rf "$COVERAGE_DIR"
        rm -f *.profraw *.profdata
        echo "✅ Rapports nettoyés"
        ;;
    *)
        echo "📊 Script de gestion du code coverage"
        echo "Usage: ./coverage.sh [command]"
        echo ""
        echo "Commandes disponibles:"
        echo "  html     - Génère un rapport HTML"
        echo "  lcov     - Génère un fichier LCOV pour VS Code"
        echo "  json     - Génère un rapport JSON"
        echo "  summary  - Affiche un résumé"
        echo "  all      - Génère tous les rapports"
        echo "  open     - Ouvre le rapport HTML dans le navigateur"
        echo "  clean    - Nettoie les rapports existants"
        echo ""
        echo "Exemple: ./coverage.sh all"
        ;;
esac
