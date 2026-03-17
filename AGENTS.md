# Repository agents guide

This repository hosts a Rust CLI for Exein Analyzer.

## Default expectations

- Prefer minimal, well-scoped changes.
- Keep `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --locked` green.
- Reuse existing command and output patterns instead of introducing parallel abstractions.
- Preserve API, command-line flags, and human-readable output unless the task explicitly changes them.

## Repository-specific guidance

- The crate is an application, so `Cargo.lock` must remain committed.
- Keep cross-platform behavior in mind: Linux, macOS, and Windows release assets are supported.
- Document any user-visible CLI change in `README.md`.
- When changing workflows, keep the GitFlow + GitVersion model aligned with `GitVersion.yml`.
