#!/usr/bin/env pwsh
[CmdletBinding()]
param(
    [string]$Runtime = $env:ANALYZER_RELEASE_RUNTIME,
    [string]$BuildNumber = $env:ANALYZER_RELEASE_BUILD,
    [string]$ReleaseDate = $env:ANALYZER_RELEASE_DATE,
    [switch]$DryRun,
    [switch]$Help
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Show-Help {
    @"
🚀 release for Windows

Synopsis:
  Produce Windows release zip archives for x86, amd64, and arm on the local host.

Usage:
  pwsh -File .\scripts\windows\release.ps1 [-BuildNumber N] [-ReleaseDate YYYYMMDD] [-DryRun] [-Help]

Artifacts:
  dist\release\windows\analyzer-cli.<Major>.<Minor>.<Release>.<Build>_<YYYYMMDD>-windows-<arch>.zip
"@ | Write-Host
}

function Write-Log([string]$Level, [string]$Message) {
    $colors = @{ INFO = "Cyan"; OK = "Green"; WARN = "Yellow"; ERR = "Red" }
    $icons = @{ INFO = "ℹ️"; OK = "✅"; WARN = "⚠️"; ERR = "❌" }
    Write-Host "$($icons[$Level]) $Message" -ForegroundColor $colors[$Level]
}

function Get-Version {
    $line = Get-Content -Path (Join-Path $repoRoot "Cargo.toml") |
        Where-Object { $_ -match '^version = "(.+)"$' } |
        Select-Object -First 1
    if (-not $line) { throw "Could not determine version from Cargo.toml." }
    return ($line -replace '^version = "(.+)"$', '$1')
}

function Get-BuildNumber {
    if ($BuildNumber) { return $BuildNumber }
    if ($env:GITHUB_RUN_NUMBER) { return $env:GITHUB_RUN_NUMBER }
    if (Get-Command git -ErrorAction SilentlyContinue) {
        return (git -C $repoRoot rev-list --count HEAD).Trim()
    }
    return "0"
}

function Get-ReleaseDate {
    if ($ReleaseDate) { return $ReleaseDate }
    return (Get-Date).ToString("yyyyMMdd")
}

function Invoke-Step([string]$Command) {
    if ($DryRun) {
        Write-Log INFO "[dry-run] $Command"
    } else {
        Invoke-Expression $Command
    }
}

function Ensure-Tool([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "$Name is required."
    }
}

function Build-Archive([string]$Arch, [string]$Target, [string]$ArchiveName) {
    $stageDir = Join-Path $stageRoot $Arch
    $binaryPath = Join-Path $repoRoot "target\$Target\release\analyzer.exe"
    Invoke-Step "cargo build --locked --release --target $Target"
    Invoke-Step "Remove-Item -Recurse -Force '$stageDir' -ErrorAction SilentlyContinue"
    Invoke-Step "New-Item -ItemType Directory -Force -Path '$stageDir' | Out-Null"
    Invoke-Step "Copy-Item '$binaryPath' (Join-Path '$stageDir' 'analyzer.exe') -Force"
    Invoke-Step "Compress-Archive -Path (Join-Path '$stageDir' 'analyzer.exe') -DestinationPath (Join-Path '$artifactDir' '$ArchiveName') -Force"
}

if ($Help) {
    Show-Help
    exit 0
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$artifactDir = Join-Path $repoRoot "dist\release\windows"
$stageRoot = Join-Path $repoRoot "dist\release\.stage-windows"
$version = Get-Version
$versionParts = $version.Split(".")
if ($versionParts.Count -lt 3) { throw "Expected semantic version in Cargo.toml." }
$major = $versionParts[0]
$minor = $versionParts[1]
$release = $versionParts[2]
$build = Get-BuildNumber
$stamp = Get-ReleaseDate
$targets = [ordered]@{
    x86 = "i686-pc-windows-msvc"
    amd64 = "x86_64-pc-windows-msvc"
    arm = "aarch64-pc-windows-msvc"
}

Ensure-Tool "cargo"
Ensure-Tool "rustup"

if ($Runtime) {
    Write-Log WARN "--runtime is ignored on Windows release packaging."
}

Invoke-Step "rustup target add i686-pc-windows-msvc x86_64-pc-windows-msvc aarch64-pc-windows-msvc"
Invoke-Step "New-Item -ItemType Directory -Force -Path '$artifactDir' | Out-Null"

foreach ($entry in $targets.GetEnumerator()) {
    $archiveName = "analyzer-cli.$major.$minor.$release.$build" + "_" + "$stamp-windows-$($entry.Key).zip"
    Write-Log INFO "Packaging $($entry.Value) -> $archiveName"
    Build-Archive -Arch $entry.Key -Target $entry.Value -ArchiveName $archiveName
}

Write-Log OK "Release archives written to $artifactDir"
