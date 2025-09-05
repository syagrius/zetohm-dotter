# Ajouter PowerShell à dotter

## Étapes réalisées

### 1. Vérifier que le profil PowerShell existe
```bash
ls -la "C:\Users\Serge\Documents\PowerShell\Microsoft.PowerShell_profile.ps1"
```
Résultat : Le fichier existe (188 bytes, modifié le 4 sept. 21:58)

### 2. Créer la structure de dossier pour PowerShell
```bash
mkdir -p powershell
```

### 3. Copier le profil PowerShell dans le dépôt dotter
```bash
cp "C:\Users\Serge\Documents\PowerShell\Microsoft.PowerShell_profile.ps1" powershell/
```

### 4. Configurer le déploiement dans .dotter/global.toml
Ajouter dans la section `[default.files]` :
```toml
"powershell/Microsoft.PowerShell_profile.ps1" = "~/Documents/PowerShell/Microsoft.PowerShell_profile.ps1"
```

### 5. Tester le déploiement
```bash
dotter deploy --dry-run
```
Résultat : Configuration détectée et prête pour le déploiement

### 6. Committer les changements
```bash
git add .
git commit -m "Add PowerShell profile configuration"
```

## Structure finale du dépôt

```
.dotter/
├── global.toml   # Contient les mappings WezTerm + PowerShell
├── local.toml    
└── cache.toml    
wezterm/
└── wezterm.lua
powershell/
└── Microsoft.PowerShell_profile.ps1    # ← Nouveau !
1st-step.md       # Documentation générale
module-powershell.md                     # ← Ce fichier
```

## Configuration résultante dans global.toml

```toml
[default.files]
"wezterm/wezterm.lua" = "~/.config/wezterm/wezterm.lua"
"powershell/Microsoft.PowerShell_profile.ps1" = "~/Documents/PowerShell/Microsoft.PowerShell_profile.ps1"
```

## Utilisation sur une nouvelle machine

1. Cloner le dépôt dotter
2. Exécuter `dotter deploy` 
3. Dotter créera automatiquement le symlink :
   - `~/Documents/PowerShell/Microsoft.PowerShell_profile.ps1` → `powershell/Microsoft.PowerShell_profile.ps1`

## Notes importantes

- Le fichier original existe déjà, donc `dotter deploy --force` sera nécessaire pour remplacer le fichier par un symlink
- Le chemin `~/Documents/PowerShell/` est le répertoire standard pour les profils PowerShell sur Windows
- Cette configuration fonctionne avec PowerShell 7+ (pas Windows PowerShell 5.1)