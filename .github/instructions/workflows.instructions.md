---
applyTo: ".github/workflows/**/*.yml"
---

# GitHub Actions instructions

- Use least-privilege permissions.
- Prefer `actions/checkout@v4` with `fetch-depth: 0` only when history is required.
- Keep workflows deterministic and compatible with GitFlow and tag-based releases.
- Use `cargo ... --locked` in CI when dependencies are expected to be reproducible.
- Add concise step names and preserve readable summaries.
