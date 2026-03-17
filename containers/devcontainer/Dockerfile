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
    && curl -fsSL https://ohmyposh.dev/install.sh | bash -s -- -d /usr/local/bin \
    && rm -rf /var/lib/apt/lists/*

COPY containers/devcontainer/post-create.sh /usr/local/share/analyzer-devcontainer/post-create.sh
RUN chmod +x /usr/local/share/analyzer-devcontainer/post-create.sh

USER vscode
WORKDIR /workspace
