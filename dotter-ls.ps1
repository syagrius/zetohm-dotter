# PowerShell script to list files deployed by dotter
# Parses global.toml file and displays file mappings

$configFile = ".\.dotter\global.toml"

if (-not (Test-Path $configFile)) {
    Write-Host "Error: File $configFile not found" -ForegroundColor Red
    exit 1
}

Write-Host "=== Files deployed by dotter ===" -ForegroundColor Cyan
Write-Host ""

# Read TOML file content
$content = Get-Content $configFile -Raw

# Simple TOML parser - extract [package.files] sections
$lines = $content -split "`n"
$currentPackage = ""
$inFilesSection = $false

foreach ($line in $lines) {
    $line = $line.Trim()
    
    # Skip empty lines and comments
    if ([string]::IsNullOrWhiteSpace($line) -or $line.StartsWith("#")) {
        continue
    }
    
    # Detect [package.files] sections
    if ($line -match '^\[([^.]+)\.files\]') {
        $currentPackage = $matches[1]
        $inFilesSection = $true
        Write-Host "Package: $currentPackage" -ForegroundColor Yellow
        continue
    }
    # Detect other sections (exit files mode)
    elseif ($line -match '^\[') {
        $inFilesSection = $false
        continue
    }
    
    # Process file mapping lines
    if ($inFilesSection -and $line -match '^"([^"]+)"\s*=\s*"([^"]+)"') {
        $source = $matches[1]
        $destination = $matches[2]
        
        # Convert ~ to Windows home path
        $destination = $destination -replace '^~', $env:USERPROFILE
        $destination = $destination -replace '/', '\'
        
        Write-Host "  $source" -ForegroundColor Green -NoNewline
        Write-Host " -> " -NoNewline
        Write-Host "$destination" -ForegroundColor Magenta
    }
}

Write-Host ""
Write-Host "=== End of list ===" -ForegroundColor Cyan