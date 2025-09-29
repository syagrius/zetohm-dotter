# Script de déploiement Dotter pour Windows
# Copie automatiquement la configuration Windows et déploie

Write-Host "Déploiement de la configuration Windows..." -ForegroundColor Green

# Copier le fichier de configuration Windows vers local.toml
Copy-Item -Path ".dotter\local-windows.toml" -Destination ".dotter\local.toml" -Force

# Exécuter dotter deploy
dotter deploy

Write-Host "Déploiement Windows terminé!" -ForegroundColor Green