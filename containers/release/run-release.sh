#!/usr/bin/env bash
# run-release.sh — cross-compile all targets and package release archives.
# Runs inside the release container. Invoked by host-side release scripts.
#
# Required environment variables:
#   ANALYZER_RELEASE_VERSION   — semver version string (e.g. "0.2.0")
#   ANALYZER_RELEASE_STAMP     — date stamp         (e.g. "20260319")
#
# Optional environment variables:
#   ANALYZER_RELEASE_BUILD     — build number, appended to version (default: 0)
#   ANALYZER_RELEASE_WORKSPACE — repo root inside container      (default: $(pwd))
set -euo pipefail

# ── Configuration ─────────────────────────────────────────────────────────────
VERSION="${ANALYZER_RELEASE_VERSION:?ANALYZER_RELEASE_VERSION is required}"
BUILD="${ANALYZER_RELEASE_BUILD:-0}"
STAMP="${ANALYZER_RELEASE_STAMP:?ANALYZER_RELEASE_STAMP is required}"
REPO_ROOT="${ANALYZER_RELEASE_WORKSPACE:-$(pwd)}"

FULL_VERSION="${VERSION}.${BUILD}"
RELEASE_SUBDIR="release_${FULL_VERSION}_${STAMP}"
ARTIFACT_DIR="${REPO_ROOT}/dist/releases/${RELEASE_SUBDIR}"

# ── Helpers ───────────────────────────────────────────────────────────────────
log()  { local c="$1"; shift; printf "\033[${c}m%s\033[0m\n" "$*"; }
info() { log 36 "ℹ️  $*"; }
ok()   { log 32 "✅ $*"; }
warn() { log 33 "⚠️  $*"; }
err()  { log 31 "❌ $*" >&2; }

# ── Cross-compilation target matrix ───────────────────────────────────────────
# Format: "os|arch|rust_target|binary_name"
TARGETS=(
  "linux|x64|x86_64-unknown-linux-gnu|analyzer"
  "linux|x86|i686-unknown-linux-gnu|analyzer"
  "linux|arm64|aarch64-unknown-linux-gnu|analyzer"
  "windows|x64|x86_64-pc-windows-gnu|analyzer.exe"
  "windows|x86|i686-pc-windows-gnu|analyzer.exe"
  "windows|arm64|aarch64-pc-windows-gnullvm|analyzer.exe"
  "macos|x64|x86_64-apple-darwin|analyzer"
  "macos|arm64|aarch64-apple-darwin|analyzer"
)

# ── Setup ─────────────────────────────────────────────────────────────────────
info "Release:  ${FULL_VERSION}"
info "Date:     ${STAMP}"
info "Output:   ${ARTIFACT_DIR}"
info "Targets:  ${#TARGETS[@]}"
echo ""

mkdir -p "$ARTIFACT_DIR"

built=0
failed=0

# ── Build and package each target ─────────────────────────────────────────────
for entry in "${TARGETS[@]}"; do
  IFS='|' read -r os arch target binary <<< "$entry"
  archive_name="exein_analyzer_cli_${FULL_VERSION}_${STAMP}_${os}_${arch}.zip"
  archive_path="${ARTIFACT_DIR}/${archive_name}"

  info "Building ${os}/${arch}  (${target})"

  rustup target add "$target" >/dev/null 2>&1 || true

  # i686-pc-windows-gnu: cargo-zigbuild's lld-link wrapper does not provide
  # the libgcc_eh symbols (___register_frame_info) referenced by rsbegin.o.
  # Use plain cargo build with the real MinGW GCC linker instead.
  if [[ "$target" == "i686-pc-windows-gnu" ]]; then
    if ! CARGO_TARGET_I686_PC_WINDOWS_GNU_LINKER=i686-w64-mingw32-gcc \
         cargo build --locked --release --target "$target" 2>&1; then
      warn "Build failed for ${target} — skipping."
      (( failed++ )) || true
      continue
    fi
  elif ! cargo zigbuild --locked --release --target "$target" 2>&1; then
    warn "Build failed for ${target} — skipping."
    (( failed++ )) || true
    continue
  fi

  # ── Stage archive contents ──────────────────────────────────────────────────
  staging="$(mktemp -d)"

  cp "${REPO_ROOT}/target/${target}/release/${binary}" "${staging}/${binary}"
  cp "${REPO_ROOT}/LICENSE"                            "${staging}/LICENSE"
  [[ -f "${REPO_ROOT}/CHANGELOG.md" ]] \
    && cp "${REPO_ROOT}/CHANGELOG.md" "${staging}/CHANGELOG.md"

  rm -f "$archive_path"
  (cd "$staging" && zip -q -r "$archive_path" .)
  rm -rf "$staging"

  ok "${archive_name}"
  (( built++ )) || true
done

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
if [[ $built -eq 0 ]]; then
  err "No targets built successfully."
  exit 1
fi

ok "Release complete — ${built} archive(s) in dist/releases/${RELEASE_SUBDIR}"
[[ $failed -gt 0 ]] && warn "${failed} target(s) failed."
exit 0
