FROM rust:1.85-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        binutils-aarch64-linux-gnu \
        binutils-i686-linux-gnu \
        bash \
        ca-certificates \
        gcc-aarch64-linux-gnu \
        gcc-i686-linux-gnu \
        git \
        gzip \
        tar \
        zip \
    && rm -rf /var/lib/apt/lists/*

COPY containers/release/run-release.sh /usr/local/bin/run-release.sh
RUN chmod +x /usr/local/bin/run-release.sh

WORKDIR /workspace
ENTRYPOINT ["/bin/bash", "/usr/local/bin/run-release.sh"]
