#!/usr/bin/env bash
set -euo pipefail

echo "🚀 Finalizing devcontainer setup..."
rustup component add rustfmt clippy
git config --global --add safe.directory /workspace
cargo fetch --locked

# ---------------------------------------------------------------------------
# Container-engine detection
# ---------------------------------------------------------------------------
# Determine which container CLI (docker or podman) can communicate with the
# socket that was mounted at /var/run/docker.sock.  Both Docker and Podman
# expose a Docker-compatible REST API, so the docker CLI works with either
# daemon.  We prefer docker if it is functional, then fall back to podman.
#
# The result is a thin wrapper /usr/local/bin/container-engine that forwards
# all arguments to the detected engine, plus a /usr/local/bin/docker symlink
# when docker itself is not the active engine (so tooling that hard-codes
# "docker" still works through the same backend).
# ---------------------------------------------------------------------------

WRAPPER=/usr/local/bin/container-engine
ACTIVE_ENGINE=""

_engine_ok() {
    "$1" info --format '{{.ServerVersion}}' >/dev/null 2>&1
}

if command -v docker >/dev/null 2>&1 && _engine_ok docker; then
    ACTIVE_ENGINE=docker
elif command -v podman >/dev/null 2>&1 && _engine_ok podman; then
    ACTIVE_ENGINE=podman
fi

if [ -n "$ACTIVE_ENGINE" ]; then
    echo "🐳 Container engine detected: $ACTIVE_ENGINE"

    # Create the container-engine wrapper.  The postCreate step runs as the
    # remoteUser (vscode), which cannot write to /usr/local/bin directly.
    # The vscode user in this base image has passwordless sudo.
    sudo tee "$WRAPPER" >/dev/null <<EOF
#!/usr/bin/env bash
exec $ACTIVE_ENGINE "\$@"
EOF
    sudo chmod +x "$WRAPPER"

    # If podman is the active engine, provide a docker shim so that tools that
    # call "docker" directly continue to work.
    if [ "$ACTIVE_ENGINE" = "podman" ] && command -v docker >/dev/null 2>&1; then
        # docker binary exists but couldn't reach a daemon; replace it with a
        # wrapper that forwards to podman.
        sudo tee /usr/local/bin/docker >/dev/null <<'EOF'
#!/usr/bin/env bash
exec podman "$@"
EOF
        sudo chmod +x /usr/local/bin/docker
        echo "   ↳ /usr/local/bin/docker shimmed → podman"
    fi
else
    echo "⚠️  No reachable container engine found."
    echo "   Make sure the host Docker or Podman socket is mounted."
    echo "   Podman users: export CONTAINER_SOCKET_PATH=<socket-path> before"
    echo "   opening the devcontainer (see .devcontainer/devcontainer.json)."
fi

echo "✅ Devcontainer is ready."
