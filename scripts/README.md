# Development bootstrap scripts

This repository includes cross-platform development bootstrap scripts under `scripts/`.

## Layout

- `scripts/windows/setup-dev.ps1` - PowerShell
- `scripts/windows/build.ps1` - PowerShell
- `scripts/windows/release.ps1` - PowerShell
- `scripts/windows/test.ps1` - PowerShell
- `scripts/macos/setup-dev` - zsh
- `scripts/macos/build` - zsh
- `scripts/macos/release` - zsh
- `scripts/macos/test` - zsh
- `scripts/linux/setup-dev` - bash
- `scripts/linux/build` - bash
- `scripts/linux/release` - bash
- `scripts/linux/test` - bash

Windows scripts use the `.ps1` extension so they can be executed directly with `pwsh -File`.

## Goals

Each `setup-dev` script is:

- idempotent
- English-only
- colorized and emoji-friendly
- self-documented with `--help`

The container runner scripts follow the same conventions.

## What `release` does

Each platform-specific `release` script packages three archives for its own platform:

- `x86`
- `amd64`
- `arm`

Archive names follow:

- `analyzer-cli.<Major>.<Minor>.<Release>.<Build>_<YYYYMMDD>-linux-<arch>.zip`
- `analyzer-cli.<Major>.<Minor>.<Release>.<Build>_<YYYYMMDD>-windows-<arch>.zip`
- `analyzer-cli.<Major>.<Minor>.<Release>.<Build>_<YYYYMMDD>-macos-<arch>.zip`

The scripts write artifacts under `dist/release/<platform>/`.

`Build` defaults to `GITHUB_RUN_NUMBER` when available, otherwise the current git commit count. You can override it with `ANALYZER_RELEASE_BUILD` or `--build-number` / `-BuildNumber`. You can override the date with `ANALYZER_RELEASE_DATE` or `--release-date` / `-ReleaseDate`.

On Windows, invoke the scripts directly with `pwsh -File`, for example:

```powershell
pwsh -File .\scripts\windows\release.ps1 -Help
pwsh -File .\scripts\windows\release.ps1 -DryRun -BuildNumber 42 -ReleaseDate 20260317
```

## What `setup-dev` does

Depending on the operating system, the script installs or configures:

- PowerShell
- Visual Studio Code
- Git
- Git LFS
- GitHub CLI
- GitFlow
- Rust toolchain
- Oh My Posh
- package managers relevant to the platform
- Docker Desktop or Podman Desktop checks, with Podman fallback

The scripts also make sure Oh My Posh starts from the current user's PowerShell profile.

## Usage

### Windows

```powershell
pwsh -File .\scripts\windows\setup-dev.ps1 --help
pwsh -File .\scripts\windows\setup-dev.ps1
```

### macOS

```bash
zsh ./scripts/macos/setup-dev --help
zsh ./scripts/macos/setup-dev
```

### Linux

```bash
bash ./scripts/linux/setup-dev --help
bash ./scripts/linux/setup-dev
```

## Notes

- Some package managers are platform-specific. Unsupported ones are reported and skipped explicitly.
- Linux support targets the most common package families: Debian/Ubuntu, Fedora/RHEL, Arch, and openSUSE.
- Container desktop installation depends on what the platform supports natively.
- The `containers/` directory defines repository-owned images for build, release, tests, and the devcontainer workflow.
