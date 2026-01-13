# Drovity Windows Installer
# Usage: irm https://raw.githubusercontent.com/MixasV/drovity/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'

Write-Host "Installing Drovity..." -ForegroundColor Cyan

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "x86" }
$repo = "MixasV/drovity"

# Get latest release
try {
    $release = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest"
    $version = $release.tag_name
    Write-Host "Latest version: $version" -ForegroundColor Green
} catch {
    Write-Host "Error: Failed to fetch latest release" -ForegroundColor Red
    exit 1
}

# Find Windows binary
$asset = $release.assets | Where-Object { $_.name -match "windows.*\.exe$" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "Error: Windows binary not found in release" -ForegroundColor Red
    exit 1
}

$downloadUrl = $asset.browser_download_url
$fileName = $asset.name

Write-Host "Downloading $fileName..." -ForegroundColor Yellow

# Create temp directory
$tempDir = Join-Path $env:TEMP "drovity-install"
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null
$tempFile = Join-Path $tempDir $fileName

# Download binary
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile -UseBasicParsing
    Write-Host "Downloaded successfully" -ForegroundColor Green
} catch {
    Write-Host "Error: Failed to download binary" -ForegroundColor Red
    exit 1
}

# Install location
$installDir = Join-Path $env:LOCALAPPDATA "Programs\drovity"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
$installPath = Join-Path $installDir "drovity.exe"

# Move binary
Move-Item -Path $tempFile -Destination $installPath -Force
Write-Host "Installed to: $installPath" -ForegroundColor Green

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$userPath;$installDir",
        "User"
    )
    Write-Host "Added to PATH (restart terminal to use 'drovity' command)" -ForegroundColor Yellow
    $env:Path += ";$installDir"
} else {
    Write-Host "Already in PATH" -ForegroundColor Green
}

# Cleanup
Remove-Item -Recurse -Force $tempDir

Write-Host "`nInstallation complete!" -ForegroundColor Green
Write-Host "Run 'drovity' to start (restart terminal if needed)" -ForegroundColor Cyan
Write-Host "`nQuick start:" -ForegroundColor Yellow
Write-Host "  1. drovity" -ForegroundColor White
Write-Host "  2. Select '1. Accounts' and add Google account" -ForegroundColor White
Write-Host "  3. Select '2. API Proxy' and start server" -ForegroundColor White
Write-Host "  4. Select '3. Droid Settings Setup' to configure Factory Droid" -ForegroundColor White
