# Contributing

Thanks for contributing to `analyzer-cli`.

## Development environment

1. Install Rust `1.85.0` or newer.
2. Clone the repository.
3. Run the validation suite before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

## Branching model

This repository follows a GitFlow-style model:

- `main`: production-ready history
- `develop`: integration branch
- `feature/<topic>`: new functionality
- `release/<version>`: release hardening
- `hotfix/<topic>`: urgent fixes from `main`

## Versioning

Version calculation is handled by `GitVersion.yml`.

- Use semantic versioning.
- Tag releases as `v<major>.<minor>.<patch>`.
- Prefer meaningful commits; `+semver:` hints are supported by GitVersion when needed.

Examples:

- `+semver: major`
- `+semver: feature`
- `+semver: fix`
- `+semver: none`

## Pull requests

- Keep changes focused and well-described.
- Update documentation for user-visible behavior changes.
- Add or adjust tests when behavior changes.
- Ensure CI passes before requesting review.

## Coding standards

- Follow existing Rust patterns in `src/`.
- Avoid broad error swallowing; prefer explicit propagation.
- Preserve CLI compatibility unless the change explicitly requires a breaking change.
- Keep output human-friendly by default and scriptable with `--format json`.

## Security

If you believe you have found a security issue, please follow [SECURITY.md](SECURITY.md) instead of opening a public issue.
