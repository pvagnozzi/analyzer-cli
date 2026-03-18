#!/usr/bin/env pwsh
[CmdletBinding()]
param(
    [switch]$Help,
    [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Script:StepCounter = 0

function Show-Help {
    @"
🚀 setup-dev for Windows

Synopsis:
  Bootstrap a professional Windows developer workstation for this repository.

Usage:
  pwsh -File .\scripts\windows\setup-dev.ps1 [--dry-run] [--help]

What it does:
  • Enables Hyper-V, Virtual Machine Platform, and WSL when possible
  • Updates WSL2 when installed and ensures an Ubuntu distro is available
  • Installs package managers when possible: Chocolatey, Scoop
  • Validates Homebrew availability and reports Windows limitations clearly
  • Installs PowerShell, Visual Studio Code, Git, Git LFS, GitHub CLI, GitFlow, Rust, and Oh My Posh
  • Configures Oh My Posh in the current user's PowerShell profile
  • Checks Docker Desktop or Podman Desktop and installs Podman when neither is present

Behavior:
  • Idempotent: safe to run multiple times
  • English output
  • Colorful logs with emoji
"@ | Write-Host
}

function Write-Log([string]$Level, [string]$Message) {
    $colors = @{
        INFO = "Cyan"
        OK   = "Green"
        WARN = "Yellow"
        ERR  = "Red"
        STEP = "Magenta"
    }

    $icons = @{
        INFO = "ℹ️"
        OK   = "✅"
        WARN = "⚠️"
        ERR  = "❌"
        STEP = "🚀"
    }

    Write-Host "$($icons[$Level]) $Message" -ForegroundColor $colors[$Level]
}

function Invoke-CommandSafe([string]$Command) {
    if ($DryRun) {
        Write-Log INFO "[dry-run] $Command"
        return
    }

    Invoke-Expression $Command
}

function Test-CommandExists([string]$Name) {
    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Start-Step([string]$Title) {
    $Script:StepCounter++
    Write-Log STEP "Step $Script:StepCounter - $Title"
}

function Test-IsAdministrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = [Security.Principal.WindowsPrincipal]::new($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Ensure-Winget {
    if (-not (Test-CommandExists "winget")) {
        throw "winget is required on Windows to install core packages."
    }

    Write-Log OK "winget is available."
}

function Ensure-WingetPackage([string]$Id, [string]$Label) {
    $alreadyInstalled = winget list --id $Id --accept-source-agreements 2>$null | Select-String -SimpleMatch $Id
    if ($alreadyInstalled) {
        Write-Log OK "$Label is already installed."
        return
    }

    Write-Log INFO "Installing $Label with winget..."
    Invoke-CommandSafe "winget install --id $Id --exact --accept-package-agreements --accept-source-agreements"
}

function Ensure-Chocolatey {
    if (Test-CommandExists "choco") {
        Write-Log OK "Chocolatey is already installed."
        return
    }

    Write-Log INFO "Installing Chocolatey..."
    Invoke-CommandSafe "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"
}

function Ensure-Scoop {
    if (Test-CommandExists "scoop") {
        Write-Log OK "Scoop is already installed."
        return
    }

    Write-Log INFO "Installing Scoop..."
    Invoke-CommandSafe "Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force; iwr -useb get.scoop.sh | iex"
}

function Ensure-Brew {
    if (Test-CommandExists "brew") {
        Write-Log OK "Homebrew is already available."
        return
    }

    if (Test-CommandExists "wsl") {
        Write-Log WARN "Homebrew is not native on Windows. Skipping automatic install and recommending WSL-based Homebrew if needed."
        return
    }

    Write-Log WARN "Homebrew is not supported natively on this Windows environment. Skipping."
}

function Ensure-WindowsOptionalFeatureEnabled([string]$FeatureName, [string]$Label) {
    try {
        $feature = Get-WindowsOptionalFeature -Online -FeatureName $FeatureName -ErrorAction Stop
    } catch {
        Write-Log WARN "$Label could not be queried on this system."
        return
    }

    if ($feature.State -eq "Enabled") {
        Write-Log OK "$Label is already enabled."
        return
    }

    if (-not (Test-IsAdministrator)) {
        Write-Log WARN "$Label requires an elevated PowerShell session to be enabled automatically."
        return
    }

    Write-Log INFO "Enabling $Label..."
    if ($DryRun) {
        Write-Log INFO "[dry-run] Enable-WindowsOptionalFeature -Online -FeatureName $FeatureName -All -NoRestart"
        return
    }

    $result = Enable-WindowsOptionalFeature -Online -FeatureName $FeatureName -All -NoRestart
    if ($result.RestartNeeded) {
        Write-Log WARN "$Label was enabled and Windows reported that a restart is required."
    } else {
        Write-Log OK "$Label was enabled."
    }
}

function Ensure-VirtualizationStack {
    Ensure-WindowsOptionalFeatureEnabled "Microsoft-Hyper-V-All" "Hyper-V"
    Ensure-WindowsOptionalFeatureEnabled "VirtualMachinePlatform" "Virtual Machine Platform"
    Ensure-WindowsOptionalFeatureEnabled "Microsoft-Windows-Subsystem-Linux" "Windows Subsystem for Linux"
    Ensure-WingetPackage "Microsoft.WSL" "WSL"
}

function Ensure-WSLAndUbuntu {
    if (-not (Test-CommandExists "wsl")) {
        Write-Log WARN "WSL is not available yet. Run the script again after the WSL package and Windows features are installed."
        return
    }

    Write-Log INFO "Setting WSL 2 as the default version..."
    Invoke-CommandSafe "wsl --set-default-version 2"

    Write-Log INFO "Updating WSL..."
    try {
        Invoke-CommandSafe "wsl --update"
        Write-Log OK "WSL update completed."
    } catch {
        Write-Log WARN "WSL update could not be completed automatically: $($_.Exception.Message)"
    }

    $distros = @()
    try {
        $distros = @(& wsl --list --quiet 2>$null) | ForEach-Object { $_.Trim() } | Where-Object { $_ }
    } catch {
        Write-Log WARN "Could not enumerate WSL distros yet."
    }

    if ($distros -match "^Ubuntu(?:-\d+\.\d+)?$") {
        Write-Log OK "An Ubuntu distro is already installed in WSL."
        return
    }

    Write-Log INFO "Installing Ubuntu for WSL..."
    if (Test-IsAdministrator) {
        try {
            Invoke-CommandSafe "wsl --install -d Ubuntu"
            return
        } catch {
            Write-Log WARN "WSL Ubuntu installation via wsl.exe failed: $($_.Exception.Message)"
        }
    }

    Ensure-WingetPackage "Canonical.Ubuntu.2204" "Ubuntu LTS"
}

function Ensure-Rust {
    if (Test-CommandExists "rustup") {
        Write-Log OK "Rustup is already installed."
        return
    }

    Write-Log INFO "Installing Rust with rustup..."
    $tempExe = Join-Path $env:TEMP "rustup-init.exe"
    Invoke-CommandSafe "Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile '$tempExe'"
    Invoke-CommandSafe "& '$tempExe' -y"
}

function Ensure-GitFlow {
    $gitFlowInstalled = $false
    if (Test-CommandExists "git-flow") {
        $gitFlowInstalled = $true
    } else {
        try {
            git flow version *> $null
            $gitFlowInstalled = $true
        } catch {
            $gitFlowInstalled = $false
        }
    }

    if ($gitFlowInstalled) {
        Write-Log OK "GitFlow is already installed."
        return
    }

    if (Test-CommandExists "choco") {
        Write-Log INFO "Installing GitFlow with Chocolatey..."
        Invoke-CommandSafe "choco install gitflow-avh -y"
        return
    }

    if (Test-CommandExists "scoop") {
        Write-Log INFO "Installing GitFlow with Scoop..."
        Invoke-CommandSafe "scoop bucket add main"
        Invoke-CommandSafe "scoop install git-flow"
        return
    }

    Write-Log WARN "Could not install GitFlow automatically because neither Chocolatey nor Scoop is available."
}

function Ensure-OhMyPoshProfile {
    if (-not (Test-CommandExists "oh-my-posh")) {
        Write-Log WARN "Oh My Posh is not installed yet. Skipping profile setup."
        return
    }

    $profilePath = & pwsh -NoProfile -Command '$PROFILE.CurrentUserAllHosts'
    $profileDir = Split-Path -Parent $profilePath
    if (-not (Test-Path $profileDir)) {
        if ($DryRun) {
            Write-Log INFO "[dry-run] New-Item -ItemType Directory -Path `"$profileDir`" -Force"
        } else {
            New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
        }
    }

    if (-not (Test-Path $profilePath)) {
        if ($DryRun) {
            Write-Log INFO "[dry-run] New-Item -ItemType File -Path `"$profilePath`" -Force"
        } else {
            New-Item -ItemType File -Path $profilePath -Force | Out-Null
        }
    }

    $profileContent = if (Test-Path $profilePath) { Get-Content $profilePath -Raw } else { "" }
    $existingLine = ($profileContent -split "`r?`n" | Where-Object { $_ -match "oh-my-posh init pwsh --config" } | Select-Object -First 1)

    if ([string]::IsNullOrWhiteSpace($existingLine)) {
        $themePath = '$env:POSH_THEMES_PATH\jandedobbeleer.omp.json'
        $initLine = "oh-my-posh init pwsh --config `"$themePath`" | Invoke-Expression"
    } else {
        $initLine = $existingLine
    }

    if ($profileContent -notmatch [regex]::Escape($initLine)) {
        Write-Log INFO "Adding Oh My Posh initialization to the current user's PowerShell profile..."
        if ($DryRun) {
            Write-Log INFO "[dry-run] Add-Content -Path `"$profilePath`" -Value `"$initLine`""
        } else {
            Add-Content -Path $profilePath -Value "`r`n$initLine`r`n"
        }
    } else {
        Write-Log OK "Oh My Posh profile initialization already exists."
    }

    if (-not $DryRun) {
        . $profilePath
        Write-Log OK "PowerShell profile reloaded."
    }
}

function Ensure-ContainerStack {
    $dockerDesktop = Test-Path "$env:ProgramFiles\Docker\Docker\Docker Desktop.exe"
    $podmanDesktop = Test-Path "$env:ProgramFiles\Podman Desktop\Podman Desktop.exe"
    $dockerCli = Test-CommandExists "docker"
    $podmanCli = Test-CommandExists "podman"

    if (($dockerDesktop -and $dockerCli) -or ($podmanDesktop -and $podmanCli)) {
        Write-Log OK "A supported container desktop stack is already installed."
        return
    }

    Write-Log INFO "Installing Podman CLI and Podman Desktop because no supported container desktop was detected."
    Ensure-WingetPackage "RedHat.Podman" "Podman CLI"
    Ensure-WingetPackage "RedHat.Podman-Desktop" "Podman Desktop"
}

if ($Help) {
    Show-Help
    exit 0
}

Start-Step "Hardening virtualization prerequisites"
Ensure-Winget
Ensure-VirtualizationStack
Ensure-WSLAndUbuntu

Start-Step "Checking package manager prerequisites"
Ensure-Winget
Ensure-Chocolatey
Ensure-Scoop
Ensure-Brew

Start-Step "Installing developer tools"
Ensure-WingetPackage "Microsoft.PowerShell" "PowerShell"
Ensure-WingetPackage "Microsoft.VisualStudioCode" "Visual Studio Code"
Ensure-WingetPackage "Git.Git" "Git"
Ensure-WingetPackage "GitHub.GitLFS" "Git LFS"
Ensure-WingetPackage "GitHub.cli" "GitHub CLI"
Ensure-WingetPackage "JanDeDobbeleer.OhMyPosh" "Oh My Posh"
Ensure-Rust
Ensure-GitFlow

Start-Step "Checking container tooling"
Ensure-ContainerStack

Start-Step "Configuring the PowerShell profile"
Ensure-OhMyPoshProfile

Write-Log OK "Windows development environment setup completed."
