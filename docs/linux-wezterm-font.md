# Installation de JetBrains Mono Nerd Font pour Wezterm (Linux)

## Problème

Wezterm affiche des erreurs ou avertissements concernant la police manquante lorsque la configuration demande **JetBrains Mono** mais que cette police n'est pas installée sur le système.

Symptômes typiques :
- Messages d'avertissement au lancement de Wezterm
- Glyphes ou icônes manquants/cassés dans le terminal
- Rendu de police incorrect ou fallback vers une police système générique
- Wezterm utilise la version "built-in" qui peut avoir des limitations

## Solution : Installation de JetBrains Mono Nerd Font

Les **Nerd Fonts** sont des polices patchées qui incluent tous les glyphes et icônes nécessaires pour un terminal moderne (icônes de fichiers, symboles Git, powerline, etc.).

### Installation automatique (méthode recommandée)

```bash
# Créer le répertoire des polices utilisateur
mkdir -p ~/.local/share/fonts

# Télécharger JetBrains Mono Nerd Font
cd ~/.local/share/fonts
wget -O JetBrainsMono.zip "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip"

# Extraire les fichiers
unzip -q -o JetBrainsMono.zip

# Nettoyer l'archive
rm JetBrainsMono.zip

# Reconstruire le cache des polices
fc-cache -fv
```

### Vérification de l'installation

```bash
# Vérifier que les polices sont bien installées
fc-list | grep -i "jetbrains" | head -5

# Vérifier que Wezterm détecte la police
wezterm ls-fonts --list-system | grep -i "jetbrains"
```

Vous devriez voir plusieurs variantes de JetBrains Mono Nerd Font listées.

### Appliquer les changements

1. **Fermer toutes les instances de Wezterm** :
   ```bash
   killall wezterm
   ```

2. **Relancer Wezterm** :
   ```bash
   wezterm
   ```

La police devrait maintenant être correctement détectée et utilisée.

## Configuration Wezterm

### Configuration de base (actuelle)

```lua
-- ~/.config/wezterm/wezterm.lua
font = wezterm.font('JetBrains Mono'),
font_size = 10.0,
```

Cette configuration fonctionne car Wezterm cherche automatiquement les variantes Nerd Font.

### Configuration explicite (recommandée)

Pour être plus explicite et garantir l'utilisation de la Nerd Font :

```lua
-- ~/.config/wezterm/wezterm.lua
font = wezterm.font('JetBrainsMono Nerd Font'),
font_size = 10.0,
```

### Configuration avec fallbacks

Pour une configuration plus robuste avec des polices de secours :

```lua
font = wezterm.font_with_fallback({
  'JetBrainsMono Nerd Font',
  'JetBrains Mono',
  'DejaVu Sans Mono',
  'Liberation Mono',
}),
```

## Alternatives

### Autres Nerd Fonts populaires

Si vous préférez une autre police, voici quelques alternatives :

#### Fira Code Nerd Font (avec ligatures)
```bash
cd ~/.local/share/fonts
wget -O FiraCode.zip "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip"
unzip -q -o FiraCode.zip
rm FiraCode.zip
fc-cache -fv
```

Configuration :
```lua
font = wezterm.font('FiraCode Nerd Font'),
harfbuzz_features = { 'calt=1', 'clig=1', 'liga=1' }, -- Activer les ligatures
```

#### Hack Nerd Font
```bash
cd ~/.local/share/fonts
wget -O Hack.zip "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Hack.zip"
unzip -q -o Hack.zip
rm Hack.zip
fc-cache -fv
```

Configuration :
```lua
font = wezterm.font('Hack Nerd Font'),
```

#### Meslo Nerd Font (excellent pour les symboles Powerline)
```bash
cd ~/.local/share/fonts
wget -O Meslo.zip "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Meslo.zip"
unzip -q -o Meslo.zip
rm Meslo.zip
fc-cache -fv
```

Configuration :
```lua
font = wezterm.font('MesloLGS Nerd Font'),
```

### Installation via gestionnaire de paquets

#### Ubuntu/Debian (polices limitées dans les dépôts)
```bash
sudo apt install fonts-jetbrains-mono
```
⚠️ **Note** : La version des dépôts Ubuntu n'est PAS une Nerd Font et n'aura pas tous les glyphes.

#### Arch Linux (AUR)
```bash
yay -S nerd-fonts-jetbrains-mono
# ou
paru -S nerd-fonts-jetbrains-mono
```

## Dépannage

### La police n'apparaît pas après installation

1. **Vérifier l'installation** :
   ```bash
   ls -la ~/.local/share/fonts/ | grep -i jet
   ```

2. **Reconstruire le cache** :
   ```bash
   fc-cache -fv
   ```

3. **Vérifier que la police est détectée** :
   ```bash
   fc-list | grep -i "JetBrains"
   ```

### Wezterm ne détecte toujours pas la police

1. **Redémarrer Wezterm complètement** :
   ```bash
   killall -9 wezterm
   wezterm
   ```

2. **Vérifier les polices disponibles dans Wezterm** :
   ```bash
   wezterm ls-fonts
   ```

3. **Tester avec une police système** :
   ```lua
   font = wezterm.font('DejaVu Sans Mono'),
   ```

### Messages d'erreur persistants

Si vous voyez encore des erreurs, essayez :

```bash
# Nettoyer complètement le cache des polices
rm -rf ~/.cache/fontconfig
fc-cache -fv

# Relancer Wezterm
killall wezterm && wezterm
```

### Problèmes de permissions

Si vous n'avez pas les permissions pour `~/.local/share/fonts/` :

```bash
# Vérifier les permissions
ls -la ~/.local/share/

# Créer le répertoire avec les bonnes permissions
mkdir -p ~/.local/share/fonts
chmod 755 ~/.local/share/fonts
```

## Ressources

- **Nerd Fonts GitHub** : https://github.com/ryanoasis/nerd-fonts
- **Nerd Fonts - Cheat Sheet** : https://www.nerdfonts.com/cheat-sheet (pour voir tous les glyphes disponibles)
- **Wezterm Documentation** : https://wezfurlong.org/wezterm/config/fonts.html
- **Liste complète des Nerd Fonts** : https://github.com/ryanoasis/nerd-fonts/releases

## Tailles approximatives

- **JetBrains Mono Nerd Font** : ~100 MB
- **Fira Code Nerd Font** : ~50 MB
- **Hack Nerd Font** : ~40 MB
- **Meslo Nerd Font** : ~30 MB

## Notes

- Les Nerd Fonts contiennent de nombreuses variantes (Light, Regular, Bold, Italic, etc.) d'où leur taille importante
- L'installation dans `~/.local/share/fonts/` ne nécessite pas les droits root
- Ces polices sont également disponibles pour d'autres terminaux (Alacritty, Kitty, iTerm2, etc.)
- Les Nerd Fonts sont compatibles avec oh-my-zsh, starship, powerlevel10k, etc.

---

**Dernière mise à jour** : 2025-10-11
**Version Nerd Fonts** : 3.1.1
