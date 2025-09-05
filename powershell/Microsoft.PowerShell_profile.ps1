fnm env --use-on-cd | Out-String | Invoke-Expression
Invoke-Expression (& { (zoxide init powershell | Out-String) })

# Ajoute Starship
Invoke-Expression (& starship init powershell)
