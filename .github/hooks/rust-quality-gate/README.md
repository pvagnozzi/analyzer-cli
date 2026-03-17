---
name: Rust quality gate
description: Run the repository quality checks after AI-driven edits.
tags: [rust, quality, copilot, hooks]
---

# Rust quality gate

This optional hook runs the Rust validation trio after Copilot or agent edits:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --locked`

Use it locally when you want fast feedback after AI-assisted changes.
