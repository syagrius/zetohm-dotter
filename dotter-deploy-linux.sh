#!/bin/bash

# Script de déploiement Dotter pour Linux
# Usage: ./dotter-deploy-linux.sh [package]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🚀 Déploiement Dotter - Configuration Linux"
echo "============================================"

# Vérifier que dotter est installé
if ! command -v dotter &> /dev/null; then
    echo "❌ Erreur: dotter n'est pas installé"
    echo "💡 Installation: cargo install dotter"
    exit 1
fi

# Déployer la configuration
if [ $# -eq 0 ]; then
    echo "📦 Déploiement du package 'linux' + dépendances..."
    dotter deploy -p linux
else
    echo "📦 Déploiement du package '$1'..."
    dotter deploy -p "$1"
fi

echo "✅ Déploiement terminé avec succès!"
echo ""
echo "📋 Pour appliquer les changements:"
echo "   • Redémarrer le terminal ou exécuter: source ~/.bashrc"
echo "   • Pour zsh: source ~/.zshrc"
echo ""
echo "🔍 Vérification des outils:"
echo "   • starship --version"
echo "   • fnm --version"
echo "   • zoxide --version"