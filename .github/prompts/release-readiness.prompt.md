---
mode: ask
description: Review the repository for release readiness before tagging a version.
---

Review this repository for release readiness.

Check:

1. `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`
2. `GitVersion.yml` and GitFlow branch assumptions
3. `.github/workflows/*.yml`
4. `README.md`, `CONTRIBUTING.md`, and `SECURITY.md`

Return:

- release blockers
- recommended follow-ups
- confidence level for tagging `v<next-version>`
