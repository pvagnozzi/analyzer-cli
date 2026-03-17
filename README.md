# Analyzer CLI

[![CI](https://github.com/exein-io/analyzer-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/exein-io/analyzer-cli/actions/workflows/ci.yml)
[![Release](https://github.com/exein-io/analyzer-cli/actions/workflows/release.yml/badge.svg)](https://github.com/exein-io/analyzer-cli/actions/workflows/release.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

`analyzer` is the official command-line interface for [Exein Analyzer](https://analyzer.exein.io), a platform for firmware and container security analysis.

It lets you authenticate, upload firmware or container images, run security analyses, inspect findings, generate compliance reports, and download SBOMs directly from the terminal.

## Features

- Authenticate against Exein Analyzer with profile-aware configuration.
- Create and manage objects representing devices or products.
- Launch firmware and container scans from the CLI.
- Inspect results for CVEs, malware, hardening, compliance, SBOMs, and more.
- Export reports and artifacts for automation pipelines.
- Use JSON output for scripting and shell integrations.

## Installation

### Homebrew

```bash
brew install exein-io/tools/analyzer
```

### Shell installer

```bash
curl -fsSL https://raw.githubusercontent.com/exein-io/analyzer-cli/main/dist/install.sh | bash
```

### Cargo

```bash
cargo install analyzer-cli
```

### From source

```bash
git clone https://github.com/exein-io/analyzer-cli.git
cd analyzer-cli
cargo install --path .
```

## Quick start

```bash
# 1. Authenticate
analyzer login

# 2. Create an object
analyzer object new "my-router"

# 3. Upload and scan a firmware image
analyzer scan new \
  --object a1b2c3d4-0000-0000-0000-000000000000 \
  --file firmware.bin \
  --type linux \
  --analysis info cve software-bom malware \
  --wait

# 4. Inspect the overview
analyzer scan overview --object a1b2c3d4-0000-0000-0000-000000000000

# 5. Download the SBOM
analyzer scan sbom --object a1b2c3d4-0000-0000-0000-000000000000 -O sbom.json
```

## Common usage

### Authentication

```bash
analyzer login
analyzer login --url https://my-analyzer.example.com/api/
analyzer login --profile staging
analyzer whoami
```

### Objects

```bash
analyzer object list
analyzer object new "my-device" --description "Router firmware" --tags iot,router
analyzer object delete <UUID>
```

### Scans

```bash
analyzer scan new -o <OBJECT_ID> -f firmware.bin -t linux -a info cve software-bom
analyzer scan new -o <OBJECT_ID> -f image.tar -t docker -a info cve malware --wait
analyzer scan status --scan <SCAN_ID>
analyzer scan score --scan <SCAN_ID>
analyzer scan results --scan <SCAN_ID> --analysis cve
analyzer scan compliance --type cra --scan <SCAN_ID>
analyzer scan report --scan <SCAN_ID> -O report.pdf --wait
analyzer scan sbom --scan <SCAN_ID> -O sbom.json
```

All commands that accept `--scan <SCAN_ID>` also accept `--object <OBJECT_ID>`, resolving the latest scan for the object automatically.

### Output formats

```bash
analyzer object list
analyzer object list --format json
analyzer scan overview --object <OBJECT_ID> --format json | jq '.analyses'
```

### Shell completions

```bash
analyzer completions bash > /etc/bash_completion.d/analyzer
analyzer completions zsh > ~/.zfunc/_analyzer
analyzer completions fish > ~/.config/fish/completions/analyzer.fish
```

## Configuration

Configuration is stored at `~/.config/analyzer/config.toml`.

```toml
default_profile = "default"

[profiles.default]
api_key = "your-api-key"
url = "https://analyzer.exein.io/api/"
```

Settings are resolved in this order:

1. CLI flags
2. Environment variables
3. Config file
4. Built-in defaults

Environment variables:

- `ANALYZER_API_KEY`
- `ANALYZER_URL`
- `ANALYZER_PROFILE`
- `ANALYZER_LANG` (`en`, `fr`, `de`, `nl`, `es`, `pt`, `zh`, `ko`, `ar`, `ja`)
- `ANALYZER_CONFIG_DIR` (optional override for the config directory)

Use `--lang <code>` to switch the human-oriented CLI theme and messages at runtime. English remains the default, while JSON output stays stable for automation.

## Supported scan types

| Type | Analyses |
|------|----------|
| `linux` | `info`, `kernel`, `cve`, `password-hash`, `crypto`, `software-bom`, `malware`, `hardening`, `capabilities` |
| `docker` | `info`, `cve`, `password-hash`, `crypto`, `software-bom`, `malware`, `hardening`, `capabilities` |
| `idf` | `info`, `cve`, `software-bom`, `symbols`, `tasks`, `stack-overflow` |

Supported compliance standard:

- `cra`

## Development

### Prerequisites

- Rust `1.85.0` or newer
- `rustfmt`
- `clippy`

### Bootstrap scripts

Repository bootstrap scripts are available under `scripts/`:

- `scripts/windows/setup-dev.ps1`
- `scripts/windows/build.ps1`
- `scripts/windows/release.ps1`
- `scripts/windows/test.ps1`
- `scripts/macos/setup-dev`
- `scripts/macos/build`
- `scripts/macos/release`
- `scripts/macos/test`
- `scripts/linux/setup-dev`
- `scripts/linux/build`
- `scripts/linux/release`
- `scripts/linux/test`

Examples:

```bash
pwsh -File .\scripts\windows\setup-dev.ps1 --help
pwsh -File .\scripts\windows\test.ps1
zsh ./scripts/macos/setup-dev --help
bash ./scripts/linux/build
bash ./scripts/linux/setup-dev --help
```

Release packaging conventions:

- `scripts/linux/release` produces Linux archives for `x86`, `amd64`, and `arm`
- `scripts/windows/release.ps1` produces Windows archives for `x86`, `amd64`, and `arm`
- `scripts/macos/release` produces macOS archives for `x86`, `amd64`, and `arm`
- archive names follow `analyzer-cli.<Major>.<Minor>.<Release>.<Build>_<YYYYMMDD>-<platform>-<arch>.zip`
- `Build` defaults to `GITHUB_RUN_NUMBER` when available, otherwise the git commit count
- use `ANALYZER_RELEASE_BUILD` or `--build-number` / `-BuildNumber`, and `ANALYZER_RELEASE_DATE` or `--release-date` / `-ReleaseDate`, to override naming metadata

### Containers and devcontainer

Repository-owned container definitions live under `containers/`:

- `containers/build`
- `containers/release`
- `containers/test`
- `containers/devcontainer`

VS Code Dev Containers can use `.devcontainer/devcontainer.json`, which is wired to `containers/devcontainer/Dockerfile`.

### Local workflow

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

`Cargo.lock` is committed because this repository ships an application, not a reusable library.

## Release and versioning

This repository follows a GitFlow-style branching strategy:

- `main` for production-ready history
- `develop` for integration
- `feature/*` for new work
- `release/*` for stabilization
- `hotfix/*` for urgent production fixes

Semantic versions are calculated with `GitVersion.yml`. Tagged releases use the `v*` convention and publish platform binaries through GitHub Actions.

## GitHub Copilot and MCP

The repository includes a curated `.github` setup with:

- repository and path-specific Copilot instructions
- reusable prompts
- custom agents for Rust maintenance and releases
- example hooks
- an `awesome-copilot` MCP server configuration

See `.github/mcp/README.md` and `.github/copilot-instructions.md` for details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before opening pull requests.

## License

Licensed under the [Apache License 2.0](LICENSE).

