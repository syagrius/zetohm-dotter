# Load FNM if package is active
$fnmInit = "$env:USERPROFILE\.config\powershell\fnm-init.ps1"
if (Test-Path $fnmInit) {
    . $fnmInit
}

# Load Zoxide if package is active
$zoxideInit = "$env:USERPROFILE\.config\powershell\zoxide-init.ps1"
if (Test-Path $zoxideInit) {
    . $zoxideInit
}

# Load Starship if package is active
$starshipInit = "$env:USERPROFILE\.config\powershell\starship-init.ps1"
if (Test-Path $starshipInit) {
    . $starshipInit
}
