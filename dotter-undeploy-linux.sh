#!/bin/bash

# Script de suppression Dotter pour Linux
# Usage: ./dotter-undeploy-linux.sh [package]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🗑️  Suppression Dotter - Configuration Linux"
echo "============================================"

# Vérifier que dotter est installé
if ! command -v dotter &> /dev/null; then
    echo "❌ Erreur: dotter n'est pas installé"
    exit 1
fi

# Supprimer la configuration
if [ $# -eq 0 ]; then
    echo "📦 Suppression du package 'linux'..."
    dotter undeploy -p linux
else
    echo "📦 Suppression du package '$1'..."
    dotter undeploy -p "$1"
fi

echo "✅ Suppression terminée avec succès!"