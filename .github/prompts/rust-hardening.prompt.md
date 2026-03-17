---
mode: edit
description: Harden Rust CLI code while preserving command-line behavior.
---

Improve reliability and maintainability for this Rust CLI without changing intended user-facing behavior.

Priorities:

1. preserve command names and flags
2. keep error handling explicit
3. avoid duplicated logic
4. update docs if behavior changes
5. keep `cargo fmt`, `cargo clippy`, and `cargo test --locked` passing
