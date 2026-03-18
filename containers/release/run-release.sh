#!/usr/bin/env bash
set -euo pipefail

repo_root="${ANALYZER_RELEASE_WORKSPACE:-$(pwd)}"
artifact_dir="$repo_root/release"
archive_name="${ANALYZER_RELEASE_ARCHIVE_NAME:-}"
binary_path_input="${ANALYZER_RELEASE_BINARY_PATH:-}"

if [[ -z "$archive_name" ]]; then
  echo "ANALYZER_RELEASE_ARCHIVE_NAME is required." >&2
  exit 1
fi

if [[ -z "$binary_path_input" ]]; then
  echo "ANALYZER_RELEASE_BINARY_PATH is required." >&2
  exit 1
fi

binary_path="$repo_root/$binary_path_input"

if [[ ! -f "$binary_path" ]]; then
  echo "Binary not found: $binary_path" >&2
  exit 1
fi

stage_dir="$(mktemp -d)"
trap 'rm -rf "$stage_dir"' EXIT

binary_name="$(basename "$binary_path")"
archive_path="$artifact_dir/$archive_name"

echo "🚀 Running release packaging in container..."
mkdir -p "$artifact_dir"
rm -f "$archive_path"
cp "$binary_path" "$stage_dir/$binary_name"
(
  cd "$stage_dir"
  zip -q -r "$archive_path" "$binary_name"
)
echo "✅ Release artifact written to $archive_path."
