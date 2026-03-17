#!/usr/bin/env bash
set -euo pipefail

echo "🚀 Finalizing devcontainer setup..."
rustup component add rustfmt clippy
git config --global --add safe.directory /workspace
cargo fetch --locked
echo "✅ Devcontainer is ready."
