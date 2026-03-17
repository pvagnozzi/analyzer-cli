#!/usr/bin/env bash
set -euo pipefail

echo "🧪 Running containerized quality gates..."
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
echo "✅ Containerized test suite completed."
