# Analyzer CLI Copilot instructions

This repository contains a Rust command-line application for Exein Analyzer.

## What matters most

- Preserve CLI compatibility and existing command names unless the task explicitly requires change.
- Keep `README.md`, release automation, and user-facing docs aligned with code changes.
- Prefer small, explicit Rust functions with `anyhow` or typed errors already used by the codebase.
- Do not swallow errors or silently ignore failed API interactions.
- Favor maintainable terminal UX: readable defaults, JSON support for automation, and clear progress/output messages.

## Validation expectations

Before considering work complete, prefer running:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --locked`

## Repository conventions

- `Cargo.lock` is committed.
- The repository follows GitFlow branch naming and GitVersion semantic versioning.
- Workflow changes should stay compatible with GitHub Actions on Linux, macOS, and Windows where applicable.
