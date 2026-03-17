#!/usr/bin/env bash
set -euo pipefail

if [[ ! -f Cargo.toml ]]; then
  exit 0
fi

cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
