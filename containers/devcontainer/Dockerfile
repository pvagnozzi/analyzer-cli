FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

USER root

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        apt-transport-https \
        bash \
        ca-certificates \
        curl \
        docker.io \
        git-flow \
        gnupg \
        jq \
        lsb-release \
        podman \
        software-properties-common \
        unzip \
        wget \
        zsh \
    && wget -q https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && rm packages-microsoft-prod.deb \
    && apt-get update \
    && apt-get install -y --no-install-recommends powershell \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js LTS (required for MCP servers via npx).
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install oh-my-posh and make themes available to all users.
RUN curl -fsSL https://ohmyposh.dev/install.sh | bash -s -- -d /usr/local/bin \
    && OMP_THEMES_SRC="$(oh-my-posh --print-install-dir 2>/dev/null || true)" \
    && mkdir -p /usr/local/share/oh-my-posh \
    && if [ -d "/root/.cache/oh-my-posh/themes" ]; then \
         cp -r /root/.cache/oh-my-posh/themes /usr/local/share/oh-my-posh/; \
       else \
         mkdir -p /usr/local/share/oh-my-posh/themes; \
       fi \
    && mkdir -p /home/vscode/.cache/oh-my-posh \
    && ln -sfn /usr/local/share/oh-my-posh/themes /home/vscode/.cache/oh-my-posh/themes \
    && chown -R vscode:vscode /home/vscode/.cache

COPY containers/devcontainer/post-create.sh /usr/local/share/analyzer-devcontainer/post-create.sh
RUN chmod +x /usr/local/share/analyzer-devcontainer/post-create.sh

USER vscode
WORKDIR /workspace
