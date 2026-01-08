# ghk installer for Windows
# Usage: irm https://raw.githubusercontent.com/bymehul/ghk/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$repo = "bymehul/ghk"
$installDir = "$env:LOCALAPPDATA\ghk"

Write-Host "Installing ghk..." -ForegroundColor Cyan

# Get latest release
$release = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest"
$asset = $release.assets | Where-Object { $_.name -like "*windows*" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "Error: Could not find Windows release" -ForegroundColor Red
    exit 1
}

# Create install directory
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

# Download
$exePath = "$installDir\ghk.exe"
Write-Host "Downloading from $($asset.browser_download_url)..."
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $exePath

Write-Host ""
Write-Host "Installed to $exePath" -ForegroundColor Green

# Add to PATH if not already there
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
    Write-Host "Added to PATH (restart terminal to use)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Run 'ghk setup' to get started!" -ForegroundColor Cyan
