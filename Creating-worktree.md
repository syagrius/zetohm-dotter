# Création d'un Git Worktree pour les Binaires

## Contexte

L'objectif est de créer une branche orpheline `zo-bin` pour gérer les binaires personnels, et utiliser un git worktree pour déployer cette branche vers `e:\zo\bin`.

## Architecture résultante

```
dotter/ (branche master)          # Dotfiles et configurations
├── .dotter/
├── powershell/
├── wezterm/
└── README.md

e:\zo\bin/ (branche zo-bin)       # Binaires via worktree
├── mon-script.ps1
├── tool.exe
└── README-bin.md
```

## Avantages de cette approche

✅ **Un seul dépôt Git** pour deux contextes distincts  
✅ **Séparation nette** : configs (master) vs binaires (zo-bin)  
✅ **Historique séparé** grâce à la branche orpheline  
✅ **Déploiement automatique** vers e:\zo\bin via worktree  
✅ **Pas de redondance** (vs solution Unison + sous-dossier)  

## Étapes de création

### 1. Créer la branche orpheline zo-bin

```bash
# Depuis le dépôt dotter
git checkout --orphan zo-bin
git rm -rf .
echo "# Mes binaires personnels" > README-bin.md
git add README-bin.md
git commit -m "Initial zo-bin branch"
```

### 2. Créer le worktree

```bash
# Créer le worktree pointant vers e:\zo\bin
git worktree add "e:\zo\bin" zo-bin
```

### 3. Vérifier la configuration

```bash
# Lister les worktrees
git worktree list

# Vérifier le contenu de e:\zo\bin
ls e:\zo\bin
```

## Workflow quotidien

### Pour les dotfiles (configs)
```bash
cd dotter/                    # Répertoire principal
git checkout master           # Branche master
# Modifier vos configs...
git add . && git commit -m "Update configs"
```

### Pour les binaires
```bash
cd e:\zo\bin                  # Worktree automatique
# Ajouter vos binaires...
git add . && git commit -m "Add new tools"
```

### Synchronisation distante
```bash
# Depuis n'importe où dans le dépôt
git push origin master zo-bin
```

## Commandes utiles

```bash
# Revenir sur master depuis n'importe quelle branche
git checkout master

# Voir l'état de tous les worktrees
git worktree list

# Supprimer un worktree (si besoin)
git worktree remove e:\zo\bin
```

## Notes

- Les deux branches ont des historiques complètement séparés
- Chaque worktree fonctionne comme un dépôt Git indépendant
- La synchronisation se fait via les commandes Git classiques
- Architecture propre sans outils externes supplémentaires