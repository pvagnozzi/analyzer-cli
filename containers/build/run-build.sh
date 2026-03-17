#!/usr/bin/env bash
set -euo pipefail

echo "🚀 Running containerized debug build..."
cargo build --locked
echo "✅ Debug build completed."
