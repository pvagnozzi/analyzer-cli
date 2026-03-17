#!/usr/bin/env bash
set -euo pipefail

repo_root="$(pwd)"
artifact_dir="$repo_root/dist/release/linux"
stage_root="$repo_root/dist/release/.stage-linux"
build_number="${ANALYZER_RELEASE_BUILD:-}"
release_date="${ANALYZER_RELEASE_DATE:-}"

parse_version() {
  local version
  version="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -n1)"
  if [[ -z "$version" ]]; then
    echo "Could not determine version from Cargo.toml." >&2
    exit 1
  fi
  printf '%s\n' "$version"
}

resolve_build_number() {
  if [[ -n "$build_number" ]]; then
    printf '%s\n' "$build_number"
    return
  fi
  if [[ -n "${GITHUB_RUN_NUMBER:-}" ]]; then
    printf '%s\n' "$GITHUB_RUN_NUMBER"
    return
  fi
  git rev-list --count HEAD 2>/dev/null || printf '0\n'
}

resolve_release_date() {
  if [[ -n "$release_date" ]]; then
    printf '%s\n' "$release_date"
    return
  fi
  date '+%Y%m%d'
}

package_target() {
  local arch="$1"
  local target="$2"
  local archive_name="$3"
  local stage_dir="$stage_root/$arch"

  case "$target" in
    i686-unknown-linux-gnu)
      CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER=i686-linux-gnu-gcc cargo build --locked --release --target "$target"
      ;;
    aarch64-unknown-linux-gnu)
      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --locked --release --target "$target"
      ;;
    *)
      cargo build --locked --release --target "$target"
      ;;
  esac

  rm -rf "$stage_dir"
  mkdir -p "$stage_dir"
  cp "target/$target/release/analyzer" "$stage_dir/analyzer"
  (
    cd "$stage_dir"
    zip -q -r "$artifact_dir/$archive_name" analyzer
  )
}

version="$(parse_version)"
IFS='.' read -r major minor release <<< "$version"
build="$(resolve_build_number)"
stamp="$(resolve_release_date)"

echo "🚀 Running Linux release packaging..."
mkdir -p "$artifact_dir"
rustup target add i686-unknown-linux-gnu x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu

package_target x86 i686-unknown-linux-gnu "analyzer-cli.${major}.${minor}.${release}.${build}_${stamp}-linux-x86.zip"
package_target amd64 x86_64-unknown-linux-gnu "analyzer-cli.${major}.${minor}.${release}.${build}_${stamp}-linux-amd64.zip"
package_target arm aarch64-unknown-linux-gnu "analyzer-cli.${major}.${minor}.${release}.${build}_${stamp}-linux-arm.zip"

for archive in "$artifact_dir"/*.zip; do
  sha256sum "$archive" > "$archive.sha256"
done

echo "✅ Linux release artifacts written to $artifact_dir."
