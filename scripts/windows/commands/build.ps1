#!/usr/bin/env pwsh
[CmdletBinding()]
param(
    [ValidateSet("docker", "podman")]
    [string]$Runtime,
    [switch]$Help,
    [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. "$PSScriptRoot\..\common\runtime.ps1"

function Show-Help {
    @"
🚀 build for Windows

Synopsis:
  Build the project inside the repository build container.

Usage:
  pwsh -File .\scripts\windows\commands\build.ps1 [-Runtime docker|podman] [-DryRun] [-Help]
"@ | Write-Host
}

if ($Help) {
    Show-Help
    exit 0
}

$containerRuntime = Resolve-Runtime
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$tag = "analyzer-cli-build:local"
$dockerfile = Join-Path $repoRoot "containers\build\Dockerfile"

Write-Log INFO "Using container runtime: $containerRuntime"
Ensure-Image -Tag $tag -Dockerfile $dockerfile
Invoke-Step "$containerRuntime run --rm -v `"${repoRoot}:/workspace`" -w /workspace $tag"
Write-Log OK "Containerized build completed."
