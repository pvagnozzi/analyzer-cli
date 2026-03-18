#!/usr/bin/env zsh
# Shared container runtime utilities sourced by macOS command scripts.
# Callers must declare: DRY_RUN=0  RUNTIME=""  before sourcing this file.

log() {
  local level="$1"
  local message="$2"
  case "$level" in
    INFO) printf '\033[36mℹ️ %s\033[0m\n' "$message" ;;
    OK)   printf '\033[32m✅ %s\033[0m\n' "$message" ;;
    WARN) printf '\033[33m⚠️ %s\033[0m\n' "$message" ;;
    ERR)  printf '\033[31m❌ %s\033[0m\n' "$message" ;;
  esac
}

run_cmd() {
  if [[ "${DRY_RUN:-0}" -eq 1 ]]; then
    log INFO "[dry-run] $*"
  else
    eval "$*"
  fi
}

resolve_runtime() {
  if [[ -n "${RUNTIME:-}" ]]; then
    printf '%s\n' "$RUNTIME"
    return
  fi
  if command -v docker >/dev/null 2>&1; then printf 'docker\n'; return; fi
  if command -v podman >/dev/null 2>&1; then printf 'podman\n'; return; fi
  log ERR "Neither docker nor podman is available."
  exit 1
}

# ensure_image TAG DOCKERFILE CONTEXT
# Builds the image only when it does not already exist locally.
ensure_image() {
  local tag="$1" dockerfile="$2" context="$3"
  if [[ "${DRY_RUN:-0}" -eq 0 ]] && "$RUNTIME" image inspect "$tag" >/dev/null 2>&1; then
    log INFO "Reusing container image: $tag"
    return
  fi
  log INFO "Building container image: $tag"
  run_cmd "$RUNTIME build -t $tag -f \"$dockerfile\" \"$context\""
}
