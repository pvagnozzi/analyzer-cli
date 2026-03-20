#!/usr/bin/env pwsh
[CmdletBinding()]
param(
    [string]$Runtime = $env:ANALYZER_RELEASE_RUNTIME,
    [string]$BuildNumber = $(if ($env:ANALYZER_RELEASE_BUILD) { $env:ANALYZER_RELEASE_BUILD } else { "0" }),
    [string]$ReleaseDate = $env:ANALYZER_RELEASE_DATE,
    [switch]$DryRun,
    [switch]$Help
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. "$PSScriptRoot\..\common\runtime.ps1"

function Show-Help {
    @"
🚀 release — cross-compile all platforms/architectures from a single container

Synopsis:
  Builds and packages release binaries for all supported OS/architecture
  combinations (Linux, macOS, Windows x64/x86/arm64) using the repository
  release container (cargo-zigbuild). All archives are created in one container
  run; no host Rust toolchain is required.

Usage:
  pwsh -File .\scripts\windows\commands\release.ps1 [-Runtime docker|podman] [-BuildNumber N] [-ReleaseDate YYYYMMDD] [-DryRun] [-Help]

Artifacts:
  dist\releases\release_<VERSION>.<BUILD>_<YYYYMMDD>\
    exein_analyzer_cli_<VERSION>.<BUILD>_<YYYYMMDD>_<OS>_<ARCH>.zip
"@ | Write-Host
}

function Get-Version {
    $line = Get-Content -Path (Join-Path $repoRoot "Cargo.toml") |
        Where-Object { $_ -match '^version = "(.+)"$' } |
        Select-Object -First 1
    if (-not $line) { throw "Could not determine version from Cargo.toml." }
    return ($line -replace '^version = "(.+)"$', '$1')
}

function Get-ReleaseDate {
    if ($ReleaseDate) { return $ReleaseDate }
    return (Get-Date).ToString("yyyyMMdd")
}

if ($Help) {
    Show-Help
    exit 0
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$artifactDir = Join-Path $repoRoot "dist\releases"
$dockerfile = Join-Path $repoRoot "containers\release\Dockerfile"
$tag = "analyzer-cli-release:local"
$containerRuntime = Resolve-Runtime
$version = Get-Version
$stamp = Get-ReleaseDate

if ($ReleaseDate) {
    Write-Log INFO "Using release date override: $ReleaseDate"
}
Write-Log INFO "Version:  $version.$BuildNumber"
Write-Log INFO "Date:     $stamp"
Write-Log INFO "Runtime:  $containerRuntime"

Invoke-Step "New-Item -ItemType Directory -Force -Path '$artifactDir' | Out-Null"
Ensure-Image -Tag $tag -Dockerfile $dockerfile

Write-Log INFO "Launching release container for all targets..."
Invoke-Step "$containerRuntime run --rm ``
  -e ANALYZER_RELEASE_VERSION=`"$version`" ``
  -e ANALYZER_RELEASE_BUILD=`"$BuildNumber`" ``
  -e ANALYZER_RELEASE_STAMP=`"$stamp`" ``
  -e ANALYZER_RELEASE_WORKSPACE=/workspace ``
  -v `"${repoRoot}:/workspace`" ``
  -w /workspace ``
  $tag"

Write-Log OK "Release complete — archives written to $artifactDir"
