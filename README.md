# 🔍 Analyzer CLI

<div align="center">

[![CI](https://github.com/exein-io/analyzer-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/exein-io/analyzer-cli/actions/workflows/ci.yml) [![Release](https://github.com/exein-io/analyzer-cli/actions/workflows/release.yml/badge.svg)](https://github.com/exein-io/analyzer-cli/actions/workflows/release.yml) [![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE) [![Crates.io](https://img.shields.io/crates/v/analyzer-cli.svg)](https://crates.io/crates/analyzer-cli) [![MSRV](https://img.shields.io/badge/MSRV-1.85.0-orange?logo=rust)](rust-toolchain.toml) [![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-CE422B?logo=rust)](https://www.rust-lang.org) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md) [![GitHub last commit](https://img.shields.io/github/last-commit/exein-io/analyzer-cli)](https://github.com/exein-io/analyzer-cli/commits/main) [![GitHub issues](https://img.shields.io/github/issues/exein-io/analyzer-cli)](https://github.com/exein-io/analyzer-cli/issues) [![Platform: Linux](https://img.shields.io/badge/platform-Linux-lightgrey?logo=linux)](https://github.com/exein-io/analyzer-cli/releases) [![Platform: macOS](https://img.shields.io/badge/platform-macOS-lightgrey?logo=apple)](https://github.com/exein-io/analyzer-cli/releases) [![Platform: Windows](https://img.shields.io/badge/platform-Windows-lightgrey?logo=windows)](https://github.com/exein-io/analyzer-cli/releases) [![Security Audit](https://img.shields.io/badge/security-audit-green?logo=dependabot)](SECURITY.md)

</div>

> **`analyzer`** is the official command-line interface for [Exein Analyzer](https://analyzer.exein.io) — a platform for firmware and container security analysis.

Authenticate, upload firmware or container images, run security analyses, inspect findings, generate compliance reports, and download SBOMs — all directly from the terminal.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🔐 **Authentication** | Profile-aware login against any Exein Analyzer instance |
| 📦 **Object management** | Create and manage logical device/product groupings |
| 🚀 **Scan launch** | Upload and trigger firmware and container image scans |
| 🔎 **Results inspection** | Browse CVEs, malware, hardening, compliance, SBOMs |
| 📄 **Reports & artifacts** | Export PDF reports, CycloneDX SBOMs, compliance reports |
| 🤖 **Automation-ready** | JSON output mode for scripting and CI/CD pipelines |

---

## 📥 Installation

### 🍺 Homebrew

```bash
brew install exein-io/tools/analyzer
```

### ⚡ Shell installer

```bash
curl -fsSL https://raw.githubusercontent.com/exein-io/analyzer-cli/main/dist/install.sh | bash
```

### 📦 Cargo

```bash
cargo install analyzer-cli
```

### 🛠️ From source

```bash
git clone https://github.com/exein-io/analyzer-cli.git
cd analyzer-cli
cargo install --path .
```

---

## 🚀 Quick start

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

---

## 📖 Common usage

### 🔐 Authentication

```bash
analyzer login
analyzer login --url https://my-analyzer.example.com/api/
analyzer login --profile staging
analyzer whoami
```

### 📦 Objects

```bash
analyzer object list
analyzer object new "my-device" --description "Router firmware" --tags iot,router
analyzer object delete <UUID>
```

### 🔬 Scans

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

> 💡 All commands that accept `--scan <SCAN_ID>` also accept `--object <OBJECT_ID>`, resolving the latest scan for the object automatically.

### 📊 Output formats

```bash
analyzer object list
analyzer object list --format json
analyzer scan overview --object <OBJECT_ID> --format json | jq '.analyses'
```

### 🐚 Shell completions

```bash
analyzer completions bash > /etc/bash_completion.d/analyzer
analyzer completions zsh > ~/.zfunc/_analyzer
analyzer completions fish > ~/.config/fish/completions/analyzer.fish
```

---

## ⚙️ Configuration

Configuration is stored at `~/.config/analyzer/config.toml`.

```toml
default_profile = "default"

[profiles.default]
api_key = "your-api-key"
url = "https://analyzer.exein.io/api/"
```

Settings are resolved in this order:

1. 🚩 CLI flags
2. 🌍 Environment variables
3. 📄 Config file
4. 🔧 Built-in defaults

**Environment variables:**

| Variable | Description |
|----------|-------------|
| `ANALYZER_API_KEY` | API key for authentication |
| `ANALYZER_URL` | Base API URL |
| `ANALYZER_PROFILE` | Active profile name |
| `ANALYZER_LANG` | Language code (`en`, `fr`, `de`, `nl`, `es`, `pt`, `zh`, `ko`, `ar`, `ja`) |
| `ANALYZER_CONFIG_DIR` | Override the configuration directory |

Use `--lang <code>` to switch the human-oriented CLI theme and messages at runtime. English is the default; JSON output stays stable for automation regardless of language.

---

## 🔬 Supported scan types

| Type | Analyses |
|------|----------|
| `linux` | `info`, `kernel`, `cve`, `password-hash`, `crypto`, `software-bom`, `malware`, `hardening`, `capabilities` |
| `docker` | `info`, `cve`, `password-hash`, `crypto`, `software-bom`, `malware`, `hardening`, `capabilities` |
| `idf` | `info`, `cve`, `software-bom`, `symbols`, `tasks`, `stack-overflow` |

**Supported compliance standards:**

| Standard | CLI value |
|----------|-----------|
| 🇪🇺 Cyber Resilience Act | `cra` |

---

## 🛠️ Development

### Prerequisites

- 🦀 Rust `1.85.0` or newer
- `rustfmt`
- `clippy`

### Bootstrap scripts

Repository bootstrap scripts are available under `scripts/`:

| Platform | Script |
|----------|--------|
| 🪟 Windows | `scripts/windows/environment/setup-dev.ps1` |
| 🍎 macOS | `scripts/macos/environment/setup-dev` |
| 🐧 Linux | `scripts/linux/environment/setup-dev` |

Command scripts (`build`, `release`, `test`) are under `scripts/<platform>/commands/`.

**Examples:**

```bash
pwsh -File .\scripts\windows\environment\setup-dev.ps1 --help
pwsh -File .\scripts\windows\commands\test.ps1
zsh ./scripts/macos/environment/setup-dev --help
bash ./scripts/linux/commands/build
```

**Release packaging conventions:**

- Archive names follow `analyzer-cli_<Version>_<YYYYMMDD>_<platform>-<arch>.zip`
- Artifacts are written under `release/`
- Use `ANALYZER_RELEASE_DATE` or `--release-date` / `-ReleaseDate` to override the date stamp
- `ANALYZER_RELEASE_BUILD` / `--build-number` remain accepted for compatibility but are ignored by the current naming convention

### 🐳 Containers and devcontainer

Repository-owned container definitions live under `containers/`:

| Directory | Purpose |
|-----------|---------|
| `containers/build` | Build environment |
| `containers/release` | Release packaging |
| `containers/test` | Test runner |
| `containers/devcontainer` | VS Code Dev Container |

VS Code Dev Containers use `.devcontainer/devcontainer.json`, wired to `containers/devcontainer/Dockerfile`.

### 🧪 Local workflow

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

> `Cargo.lock` is committed because this repository ships an application, not a reusable library.

---

## 🏷️ Release and versioning

This repository follows a **GitFlow-style** branching strategy:

| Branch | Purpose |
|--------|---------|
| `main` | Production-ready history |
| `develop` | Integration branch |
| `feature/*` | New functionality |
| `release/*` | Release hardening |
| `hotfix/*` | Urgent production fixes |

Semantic versions are calculated with `GitVersion.yml`. Tagged releases use the `v*` convention and publish platform binaries through GitHub Actions.

---

## 🤖 GitHub Copilot and MCP

The repository includes a curated `.github` setup with:

- 📋 Repository and path-specific Copilot instructions
- 🔄 Reusable prompts
- 🤖 Custom agents for Rust maintenance and releases
- 🪝 Example hooks
- 🔌 An `awesome-copilot` MCP server configuration

---

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## 🔒 Security

Found a vulnerability? Please follow the responsible disclosure process in [SECURITY.md](SECURITY.md).

## 📜 License

This project is licensed under the [Apache 2.0 License](LICENSE).

See `.github/mcp/README.md` and `.github/copilot-instructions.md` for details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before opening pull requests.

## License

Licensed under the [Apache License 2.0](LICENSE).

