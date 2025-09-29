@echo off
REM Script de déploiement Dotter pour Windows (Batch)
REM Copie automatiquement la configuration Windows et déploie

echo Deploiement de la configuration Windows...

REM Copier le fichier de configuration Windows vers local.toml
copy /Y ".dotter\local-windows.toml" ".dotter\local.toml" >nul

REM Exécuter dotter deploy
dotter deploy

echo Deploiement Windows termine!
echo.