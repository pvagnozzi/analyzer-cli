# 🤝 Contributing

Thanks for contributing to `analyzer-cli`! Every bug report, feature suggestion, and pull request is appreciated.

---

## 🛠️ Development environment

1. Install Rust `1.85.0` or newer.
2. Clone the repository.
3. Run the validation suite before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

---

## 🌿 Branching model

This repository follows a **GitFlow-style** model:

| Branch | Purpose |
|--------|---------|
| `main` | Production-ready history |
| `develop` | Integration branch |
| `feature/<topic>` | New functionality |
| `release/<version>` | Release hardening |
| `hotfix/<topic>` | Urgent fixes from `main` |

---

## 🏷️ Versioning

Version calculation is handled by `GitVersion.yml`.

- Use semantic versioning.
- Tag releases as `v<major>.<minor>.<patch>`.
- Prefer meaningful commits; `+semver:` hints are supported by GitVersion when needed.

**`+semver:` commit hints:**

| Hint | Effect |
|------|--------|
| `+semver: major` | Bumps major version |
| `+semver: feature` | Bumps minor version |
| `+semver: fix` | Bumps patch version |
| `+semver: none` | No version bump |

---

## 📬 Pull requests

- ✅ Keep changes focused and well-described.
- ✅ Update documentation for user-visible behavior changes.
- ✅ Add or adjust tests when behavior changes.
- ✅ Ensure CI passes before requesting review.

---

## 🦀 Coding standards

- Follow existing Rust patterns in `src/`.
- Avoid broad error swallowing; prefer explicit propagation.
- Preserve CLI compatibility unless the change explicitly requires a breaking change.
- Keep output human-friendly by default and scriptable with `--format json`.

---

## 🔒 Security

If you believe you have found a security issue, please follow [SECURITY.md](SECURITY.md) instead of opening a public issue.
