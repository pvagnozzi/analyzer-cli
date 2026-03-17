---
applyTo: "src/**/*.rs"
---

# Rust source instructions

- Follow existing `clap` patterns for commands, flags, and subcommands.
- Prefer explicit types and builder-free clarity over clever abstractions.
- Keep I/O and API errors visible to users with actionable context.
- Reuse existing client, config, and output modules before introducing new helpers.
- When adding new commands or flags, update `README.md`.
