# Load FNM if package is active
$fnmInit = "$env:USERPROFILE\.config\powershell\fnm-init.ps1"
if (Test-Path $fnmInit) {
    . $fnmInit
}

Invoke-Expression (& { (zoxide init powershell | Out-String) })

# Add Starship
Invoke-Expression (& starship init powershell)
