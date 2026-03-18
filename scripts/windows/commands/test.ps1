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
🚀 test for Windows

Synopsis:
  Run formatting, linting, and tests inside the repository test container.

Usage:
  pwsh -File .\scripts\windows\commands\test.ps1 [-Runtime docker|podman] [-DryRun] [-Help]
"@ | Write-Host
}

if ($Help) {
    Show-Help
    exit 0
}

$containerRuntime = Resolve-Runtime
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$tag = "analyzer-cli-test:local"
$dockerfile = Join-Path $repoRoot "containers\test\Dockerfile"

Write-Log INFO "Using container runtime: $containerRuntime"
Ensure-Image -Tag $tag -Dockerfile $dockerfile
Invoke-Step "$containerRuntime run --rm -v `"${repoRoot}:/workspace`" -w /workspace $tag"
Write-Log OK "Containerized test run completed."
