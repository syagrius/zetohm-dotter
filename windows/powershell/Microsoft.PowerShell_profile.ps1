# Load FNM if package is active
$fnmInit = "$env:USERPROFILE\.config\powershell\fnm-init.ps1"
if (Test-Path $fnmInit) {
    . $fnmInit
}

Invoke-Expression (& { (zoxide init powershell | Out-String) })

# Load Starship if package is active
$starshipInit = "$env:USERPROFILE\.config\powershell\starship-init.ps1"
if (Test-Path $starshipInit) {
    . $starshipInit
}
