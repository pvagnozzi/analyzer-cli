# Development bootstrap scripts

This repository includes cross-platform development bootstrap scripts under `scripts/`.

## Layout

Scripts are organised per operating system and then by topic:

```
scripts/
  linux/
    commands/    build  release  test         # container-backed operations
    common/      runtime.sh                   # shared bash utilities
    environment/ setup-dev                    # workstation bootstrap
  macos/
    commands/    build  release  test         # container-backed operations
    common/      runtime.sh                   # shared zsh utilities
    environment/ setup-dev                    # workstation bootstrap
  windows/
    commands/    build.ps1  release.ps1  test.ps1   # container-backed operations
    common/      runtime.ps1                         # shared PowerShell utilities
    environment/ setup-dev.ps1                       # workstation bootstrap
```

Windows scripts use the `.ps1` extension so they can be executed directly with `pwsh -File`.

## Goals

Each `setup-dev` script is:

- idempotent
- English-only
- colorized and emoji-friendly
- self-documented with `--help`

The container runner scripts follow the same conventions and build their images on demand when the local image does not exist yet.

## What `release` does

Each platform-specific `release` script builds all supported architectures for that platform and
packages each binary into a separate archive.

Archive names follow:

- `analyzer-cli_<Version>_<YYYYMMDD>_linux-<arch>.zip`
- `analyzer-cli_<Version>_<YYYYMMDD>_windows-<arch>.zip`
- `analyzer-cli_<Version>_<YYYYMMDD>_macos-<arch>.zip`

Supported architectures per platform:

| Platform | amd64 | x86 | arm / arm64 |
|----------|-------|-----|-------------|
| Linux    | ✅    | ✅  | ✅ (requires `aarch64-linux-gnu-gcc`) |
| macOS    | ✅    | —   | ✅          |
| Windows  | ✅    | ✅  | ✅ (requires MSVC ARM64 toolchain)    |

The scripts write artifacts under `release/`.

`ANALYZER_RELEASE_DATE` or `--release-date` / `-ReleaseDate` can override the date stamp. `ANALYZER_RELEASE_BUILD` and `--build-number` / `-BuildNumber` remain accepted for compatibility, but they are ignored by the current archive naming convention.

On Windows, invoke the scripts directly with `pwsh -File`, for example:

```powershell
pwsh -File .\scripts\windows\commands\release.ps1 -Help
pwsh -File .\scripts\windows\commands\release.ps1 -DryRun -BuildNumber 42 -ReleaseDate 20260317
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
# Environment setup
pwsh -File .\scripts\windows\environment\setup-dev.ps1 --help
pwsh -File .\scripts\windows\environment\setup-dev.ps1

# Build / test / release
pwsh -File .\scripts\windows\commands\build.ps1
pwsh -File .\scripts\windows\commands\test.ps1
pwsh -File .\scripts\windows\commands\release.ps1
```

### macOS

```bash
# Environment setup
zsh ./scripts/macos/environment/setup-dev --help
zsh ./scripts/macos/environment/setup-dev

# Build / test / release
zsh ./scripts/macos/commands/build
zsh ./scripts/macos/commands/test
zsh ./scripts/macos/commands/release
```

### Linux

```bash
# Environment setup
bash ./scripts/linux/environment/setup-dev --help
bash ./scripts/linux/environment/setup-dev

# Build / test / release
bash ./scripts/linux/commands/build
bash ./scripts/linux/commands/test
bash ./scripts/linux/commands/release
```

## Notes

- Some package managers are platform-specific. Unsupported ones are reported and skipped explicitly.
- Linux support targets the most common package families: Debian/Ubuntu, Fedora/RHEL, Arch, and openSUSE.
- Container desktop installation depends on what the platform supports natively.
- The `containers/` directory defines repository-owned images for build, release, tests, and the devcontainer workflow.
