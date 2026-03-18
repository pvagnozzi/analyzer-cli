#!/usr/bin/env bash
# Shared container runtime utilities sourced by Linux command scripts.
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

# Convert an absolute host path to a form the container runtime can mount.
# On WSL/Cygwin with a Windows-native runtime (docker.exe / podman.exe) the
# path must be in Windows format; everywhere else it is returned unchanged.
normalize_host_path() {
  local path="$1"
  if [[ "${RUNTIME:-}" == *.exe ]]; then
    if command -v wslpath >/dev/null 2>&1; then wslpath -w "$path"; return; fi
    if command -v cygpath >/dev/null 2>&1; then cygpath -aw "$path"; return; fi
  fi
  printf '%s\n' "$path"
}

# Join a host-format base path with a forward-slash relative suffix.
host_path_join() {
  local base="$1" suffix="$2"
  if [[ "${RUNTIME:-}" == *.exe ]]; then
    printf '%s\\%s\n' "$base" "${suffix//\//\\}"
    return
  fi
  printf '%s/%s\n' "$base" "$suffix"
}

resolve_runtime() {
  if [[ -n "${RUNTIME:-}" ]]; then
    case "$RUNTIME" in
      podman)
        command -v podman     >/dev/null 2>&1 && { printf 'podman\n';     return; }
        command -v podman.exe >/dev/null 2>&1 && { printf 'podman.exe\n'; return; }
        ;;
      docker)
        command -v docker     >/dev/null 2>&1 && { printf 'docker\n';     return; }
        command -v docker.exe >/dev/null 2>&1 && { printf 'docker.exe\n'; return; }
        ;;
      *)
        printf '%s\n' "$RUNTIME"; return ;;
    esac
  fi
  command -v docker     >/dev/null 2>&1 && { printf 'docker\n';     return; }
  command -v docker.exe >/dev/null 2>&1 && { printf 'docker.exe\n'; return; }
  command -v podman     >/dev/null 2>&1 && { printf 'podman\n';     return; }
  command -v podman.exe >/dev/null 2>&1 && { printf 'podman.exe\n'; return; }
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
