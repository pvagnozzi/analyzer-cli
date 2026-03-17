FROM rust:1.85-bookworm

RUN rustup component add clippy rustfmt

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        bash \
        ca-certificates \
        git \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY containers/test/run-tests.sh /usr/local/bin/run-tests.sh
RUN chmod +x /usr/local/bin/run-tests.sh

WORKDIR /workspace
ENTRYPOINT ["/usr/local/bin/run-tests.sh"]
