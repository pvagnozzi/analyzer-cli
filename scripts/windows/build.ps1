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

function Show-Help {
    @"
🚀 build for Windows

Synopsis:
  Build the project inside the repository build container.

Usage:
  pwsh -File .\scripts\windows\build.ps1 [--runtime docker|podman] [--dry-run] [--help]
"@ | Write-Host
}

function Write-Log([string]$Level, [string]$Message) {
    $colors = @{ INFO = "Cyan"; OK = "Green"; WARN = "Yellow"; ERR = "Red" }
    $icons = @{ INFO = "ℹ️"; OK = "✅"; WARN = "⚠️"; ERR = "❌" }
    Write-Host "$($icons[$Level]) $Message" -ForegroundColor $colors[$Level]
}

function Resolve-Runtime {
    if ($Runtime) { return $Runtime }
    if (Get-Command docker -ErrorAction SilentlyContinue) { return "docker" }
    if (Get-Command podman -ErrorAction SilentlyContinue) { return "podman" }
    throw "Neither docker nor podman is available."
}

function Invoke-ContainerCommand([string]$Command) {
    if ($DryRun) {
        Write-Log INFO "[dry-run] $Command"
    } else {
        Invoke-Expression $Command
        if ($LASTEXITCODE -ne 0) {
            throw "Container command failed with exit code $LASTEXITCODE."
        }
    }
}

if ($Help) {
    Show-Help
    exit 0
}

$containerRuntime = Resolve-Runtime
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$tag = "analyzer-cli-build:local"
$dockerfile = Join-Path $repoRoot "containers\build\Dockerfile"

Write-Log INFO "Using container runtime: $containerRuntime"
Invoke-ContainerCommand "$containerRuntime build -t $tag -f `"$dockerfile`" `"$repoRoot`""
Invoke-ContainerCommand "$containerRuntime run --rm -v `"${repoRoot}:/workspace`" -w /workspace $tag"
Write-Log OK "Containerized build completed."

