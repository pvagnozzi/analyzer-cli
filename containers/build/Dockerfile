FROM rust:1.85-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        bash \
        ca-certificates \
        curl \
        git \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY containers/build/run-build.sh /usr/local/bin/run-build.sh
RUN chmod +x /usr/local/bin/run-build.sh

WORKDIR /workspace
ENTRYPOINT ["/usr/local/bin/run-build.sh"]
