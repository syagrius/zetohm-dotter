#!/bin/bash
# Script de déploiement Dotter pour Linux
# Copie automatiquement la configuration Linux et déploie

echo -e "\033[32mDéploiement de la configuration Linux...\033[0m"

# Copier le fichier de configuration Linux vers local.toml
cp .dotter/local-linux.toml .dotter/local.toml

# Exécuter dotter deploy
dotter deploy

echo -e "\033[32mDéploiement Linux terminé!\033[0m"