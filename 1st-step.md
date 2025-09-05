# Ajouter un outil à dotter - Guide étape par étape

## Exemple : Configuration WezTerm

### 1. Créer la structure de dossier
```bash
mkdir -p wezterm
```

### 2. Copier le fichier de configuration
```bash
cp "C:\Users\Serge\.config\wezterm\wezterm.lua" wezterm/
```

### 3. Configurer le déploiement dans .dotter/global.toml
Ajouter dans la section `[default.files]` :
```toml
"wezterm/wezterm.lua" = "~/.config/wezterm/wezterm.lua"
```

### 4. Tester le déploiement
```bash
dotter deploy --dry-run
```

### 5. Déployer (au choix)
```bash
# Option 1 : Supprimer l'ancien fichier puis déployer
rm "C:\Users\Serge\.config\wezterm\wezterm.lua"
dotter deploy

# Option 2 : Forcer le remplacement
dotter deploy --force
```

### 6. Committer les changements
```bash
git add .
git commit -m "Add wezterm configuration"
```

## Processus général pour d'autres outils

1. **Créer un dossier** pour l'outil dans le dépôt dotter
2. **Copier les fichiers** de configuration de leur emplacement actuel
3. **Modifier .dotter/global.toml** pour ajouter le mapping source → destination
4. **Tester avec --dry-run** avant le déploiement réel
5. **Déployer** avec `dotter deploy` (ou `--force` si nécessaire)
6. **Committer** les changements dans Git

## Format du mapping dans global.toml
```toml
[default.files]
"dossier-source/fichier" = "~/chemin/de/destination"
```

Le chemin source est relatif au dépôt dotter, le chemin destination utilise `~` pour le home directory.