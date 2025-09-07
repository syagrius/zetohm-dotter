    Innovation in progress • Test it, comment, contribute!
    🇫🇷 Documentation and discussions in French

Système de gestion des configurations avec Dotter

## 📋 Description

Ce projet expérimental présente un système de gestion des fichiers de configuration utilisant Dotter en remplacement des git worktrees pour versionner séparément chaque outil de développement. Cette approche permet de maintenir des configurations isolées tout en conservant un historique distinct pour chaque composant de la toolchain.

## 🏢 Contexte - Zet'ohm 2025

Depuis 2025, chez Zet'ohm, nous intégrons Rust dans notre écosystème de développement. Notre philosophie combine :

    Open-source : pour la collaboration et la transparence
    Closed-source : pour certains développements spécifiques

Ces deux approches se complètent parfaitement dans notre stratégie technique.
## 🔄 Évolution de la toolchain
Stack d'origine

    Backend : Lazarus/FPC et Python 3
    Infrastructure : Serveurs bare metal (OVH Cloud, Kimsufi, SoYouStart, nFrance) sous Debian + WSL2 pour Windows Insiders
    Client : Delphi
    Terminal : ConEmu, Terminal Preview
    Environnement Python : Miniconda 3
    Conteneurisation : Docker

Nouvelle stack (2025+)
Conservé et amélioré

    Lazarus avec mORMot2 + Zeos
    Principe du dog fooding : utilisation de nos outils internes avec la toolchain déployée chez nos utilisateurs ou clients

Nouveautés et remplacements

    Terminal : WezTerm (Rust) + Sharship (Rust) + zoxide (Rust) + fzf (go) → remplace ConEmu + Terminal Preview
    Microservices : Nouveaux services en Rust
    Node.js : fnm (Rust) pour la gestion des versions Node.js
    IA : Claude CLI pour l'intégration IA
    Frontend : Svelte/SvelteKit pour l'expérience UX/UI
    Shell : PowerShell 7
    Python : Conda impérativement en version 25.x ⚠️ Penser à exporter les environnements avant mise à jour
    WebView : Migration de DCEF vers WebView2 pour les nouveaux développements
    Conteneurs : Podman en complément/concurrent de Docker
    Installeurs : Conservation d'InnoSetup

## Installation

### 1. Premier setup sur une nouvelle machine

```powershell
# Prérequis
# Le PC doit être en mode développeur pour supporter les symlinks gérés par dotter

# gsudo pour l'élévation en ligne de commande
scoop install gsudo
# miniconda:  choco obsolete et scoop pb de path
winget install miniconda3
choco install weztree
winget install microsoft.powershell
conda init powershell

scoop install zoxide fzf
zoxide init powershell
# Pour l'élévation
winget install gsudo
