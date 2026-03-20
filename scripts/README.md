# 🛠️ Development bootstrap scripts

This repository includes cross-platform development bootstrap scripts under `scripts/`.

## 📁 Layout

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

---

## 🎯 Goals

Each `setup-dev` script is:

- ♻️ **Idempotent** — safe to run multiple times
- 🌍 **English-only** — all output in English
- 🎨 **Colorized and emoji-friendly** — readable terminal UX
- 📖 **Self-documented** — includes `--help`

The container runner scripts follow the same conventions and build their images on demand when the local image does not exist yet.

---

## 📦 What `release` does

Any platform-specific `release` script (Linux, macOS, or Windows) builds **all** supported
OS/architecture combinations in a single container run using
[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) + Zig as the universal linker.
No host Rust toolchain is required — the container handles everything.

**Output folder structure:**

```
dist/releases/
  release_<VERSION>.<BUILD>_<YYYYMMDD>/
    exein_analyzer_cli_<VERSION>.<BUILD>_<YYYYMMDD>_<OS>_<ARCH>.zip
      analyzer   (or  analyzer.exe  on Windows targets)
      LICENSE
      CHANGELOG.md
```

**Supported targets (all produced in one run):**

| OS | Architecture | Rust target |
|----|-------------|-------------|
| 🐧 linux   | x64   | `x86_64-unknown-linux-gnu`    |
| 🐧 linux   | x86   | `i686-unknown-linux-gnu`      |
| 🐧 linux   | arm64 | `aarch64-unknown-linux-gnu`   |
| 🪟 windows | x64   | `x86_64-pc-windows-gnu`       |
| 🪟 windows | x86   | `i686-pc-windows-gnu`         |
| 🪟 windows | arm64 | `aarch64-pc-windows-gnullvm`  |
| 🍎 macos   | x64   | `x86_64-apple-darwin`         |
| 🍎 macos   | arm64 | `aarch64-apple-darwin`        |

**Environment variables / flags:**

| Variable / flag | Default | Description |
|----------------|---------|-------------|
| `ANALYZER_RELEASE_BUILD` / `--build-number` | `0` | Build number appended to the version |
| `ANALYZER_RELEASE_DATE` / `--release-date` | today | Override the `YYYYMMDD` date stamp |
| `--dry-run` / `-DryRun` | off | Print commands without executing them |

On Windows, invoke the script directly with `pwsh -File`:

```powershell
pwsh -File .\scripts\windows\commands\release.ps1 -Help
pwsh -File .\scripts\windows\commands\release.ps1 -DryRun -BuildNumber 42 -ReleaseDate 20260317
```

---

## 🚀 What `setup-dev` does

Depending on the operating system, the script installs or configures:

- 💻 PowerShell
- 🖊️ Visual Studio Code
- 🔀 Git
- 📦 Git LFS
- 🐙 GitHub CLI
- 🌿 GitFlow
- 🦀 Rust toolchain
- 🎨 Oh My Posh
- 📋 Package managers relevant to the platform
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
