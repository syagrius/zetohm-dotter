# Module FNM (Fast Node Manager)

## Description

Ce module permet d'intégrer **FNM (Fast Node Manager)** dans votre configuration dotter de manière modulaire. FNM est un gestionnaire de versions Node.js rapide et simple, écrit en Rust.

### Fonctionnalités

- ✅ **Activation/désactivation facile** via les packages dotter
- ✅ **Configuration modulaire** séparée du profil PowerShell principal
- ✅ **Chargement automatique** des versions Node.js selon les fichiers `.nvmrc`
- ✅ **Performance optimale** grâce à l'architecture modulaire

## Architecture

```
dotter/
├── fnm/
│   └── fnm-init.ps1                    # Script d'initialisation FNM
├── powershell/
│   └── Microsoft.PowerShell_profile.ps1  # Profil principal (sourcing conditionnel)
└── .dotter/
    ├── global.toml                     # Définition du package fnm
    └── local.toml                      # Activation des packages
```

### Fonctionnement

1. **Profil PowerShell principal** vérifie si `~/.config/powershell/fnm-init.ps1` existe
2. **Si le package fnm est actif** → le fichier existe et FNM se charge
3. **Si le package fnm est inactif** → le fichier n'existe pas, FNM ignoré

## Utilisation

### Activer le module FNM

```toml
# Dans .dotter/local.toml
packages = ["default", "fnm"]
```

Puis déployer :
```bash
dotter deploy
```

### Désactiver le module FNM

```toml
# Dans .dotter/local.toml  
packages = ["default"]
```

Puis déployer :
```bash
dotter deploy
```

## Installation - Étapes réalisées

### 1. Ajout du package dans global.toml

```toml
# Package FNM (Fast Node Manager)
[fnm]
depends = []

[fnm.files]
"fnm/fnm-init.ps1" = "~/.config/powershell/fnm-init.ps1"

[fnm.variables]
```

### 2. Création du dossier et fichier d'initialisation

```bash
mkdir fnm
```

**Fichier `fnm/fnm-init.ps1`** :
```powershell
# FNM (Fast Node Manager) initialization for PowerShell
# This file is managed by dotter - package: fnm

fnm env --use-on-cd | Out-String | Invoke-Expression
```

### 3. Modification du profil PowerShell principal

**Avant** (ligne directe) :
```powershell
fnm env --use-on-cd | Out-String | Invoke-Expression
```

**Après** (sourcing conditionnel) :
```powershell
# Load FNM if package is active
$fnmInit = "$env:USERPROFILE\.config\powershell\fnm-init.ps1"
if (Test-Path $fnmInit) {
    . $fnmInit
}
```

### 4. Activation du package

```toml
# Dans .dotter/local.toml
packages = ["default", "fnm"]
```

### 5. Tests de validation

- ✅ **Activation** : `packages = ["default", "fnm"]` → FNM actif
- ✅ **Désactivation** : `packages = ["default"]` → FNM inactif
- ✅ **Déploiement** : `dotter deploy` fonctionne sans erreur
- ✅ **Fichiers** : Liens symboliques créés/supprimés correctement

## Commandes utiles

```bash
# Lister les fichiers déployés
powershell -ExecutionPolicy Bypass -File "dotter-ls.ps1"

# Déployer la configuration
dotter deploy

# Vérifier l'état du package fnm
ls -la ~/.config/powershell/
```

## Extensions possibles

Ce modèle peut être étendu pour d'autres outils :
- **Volta** (`volta/volta-init.ps1`)  
- **NVM** (`nvm/nvm-init.ps1`)
- **PNPM** (`pnpm/pnpm-init.ps1`)
- **Yarn** (`yarn/yarn-init.ps1`)

Chaque outil devient un package indépendant, activable/désactivable individuellement.