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

. "$PSScriptRoot\..\common\runtime.ps1"

function Show-Help {
    @"
🚀 release for Windows

Synopsis:
  Build Windows release binaries for all supported architectures and package each
  one using the repository release container.

Usage:
  pwsh -File .\scripts\windows\commands\release.ps1 [-Runtime docker|podman] [-BuildNumber N] [-ReleaseDate YYYYMMDD] [-DryRun] [-Help]

Artifacts:
  release\analyzer-cli_<Version>_<YYYYMMDD>_windows-<arch>.zip
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

function Get-AllTargets {
    return @(
        @{ Arch = 'amd64'; Target = 'x86_64-pc-windows-msvc' },
        @{ Arch = 'x86';   Target = 'i686-pc-windows-msvc' },
        @{ Arch = 'arm64'; Target = 'aarch64-pc-windows-msvc' }
    )
}

function Ensure-Tool([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "$Name is required."
    }
}

if ($Help) {
    Show-Help
    exit 0
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$artifactDir = Join-Path $repoRoot "release"
$dockerfile = Join-Path $repoRoot "containers\release\Dockerfile"
$tag = "analyzer-cli-release:local"
$containerRuntime = Resolve-Runtime
$version = Get-Version
$stamp = Get-ReleaseDate

if ($BuildNumber) {
    Write-Log WARN "Build number metadata is ignored by the current archive naming convention."
}
if ($ReleaseDate) {
    Write-Log INFO "Using release date override: $ReleaseDate"
}

Ensure-Tool "cargo"
Ensure-Tool "rustup"

Write-Log INFO "Using container runtime: $containerRuntime"
Invoke-Step "New-Item -ItemType Directory -Force -Path '$artifactDir' | Out-Null"
Ensure-Image -Tag $tag -Dockerfile $dockerfile

$builtCount = 0
foreach ($t in Get-AllTargets) {
    try {
        Invoke-Step "rustup target add $($t.Target)"
        if ($DryRun) {
            Write-Log INFO "[dry-run] cargo build --locked --release --target $($t.Target)"
        } else {
            cargo build --locked --release --target $t.Target
            if ($LASTEXITCODE -ne 0) { throw "Build failed with exit code $LASTEXITCODE." }
        }
    } catch {
        Write-Log WARN "Skipping $($t.Target): $_"
        continue
    }
    $archiveName = "analyzer-cli_{0}_{1}_windows-{2}.zip" -f $version, $stamp, $t.Arch
    $binaryPath = "target/$($t.Target)/release/analyzer.exe"
    Write-Log INFO "Packaging $($t.Target) -> $archiveName"
    Invoke-Step "$containerRuntime run --rm -e ANALYZER_RELEASE_ARCHIVE_NAME=`"$archiveName`" -e ANALYZER_RELEASE_BINARY_PATH=`"$binaryPath`" -v `"${repoRoot}:/workspace`" -w /workspace $tag"
    $builtCount++
}

if ($builtCount -eq 0 -and -not $DryRun) {
    Write-Log ERR "No targets were built successfully."
    exit 1
}

Write-Log OK "Release archives written to $artifactDir"
