FROM rust:1.85-bookworm

# ── System tools & cross-compilation prerequisites ────────────────────────────
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        bash \
        build-essential \
        ca-certificates \
        cmake \
        curl \
        git \
        libc6-dev-i386 \
        mingw-w64 \
        pkg-config \
        python3 \
        xz-utils \
        zip \
    && rm -rf /var/lib/apt/lists/*

# ── Zig (universal cross-linker used by cargo-zigbuild) ───────────────────────
ARG ZIG_VERSION=0.13.0
RUN ARCH="$(uname -m)" \
    && URL="https://ziglang.org/download/${ZIG_VERSION}/zig-linux-${ARCH}-${ZIG_VERSION}.tar.xz" \
    && curl -fsSL "$URL" -o /tmp/zig.tar.xz \
    && tar -xf /tmp/zig.tar.xz -C /usr/local \
    && ln -s "/usr/local/zig-linux-${ARCH}-${ZIG_VERSION}/zig" /usr/local/bin/zig \
    && rm /tmp/zig.tar.xz \
    && zig version

# ── cargo-zigbuild ────────────────────────────────────────────────────────────
RUN cargo install cargo-zigbuild --locked --version "^0.19"

# ── macOS SDK (CoreFoundation and other Apple frameworks) ─────────────────────
# Provides the framework stubs needed when cross-compiling for Darwin targets.
# SDK redistribution sourced from https://github.com/phracker/MacOSX-SDKs
RUN curl -fsSL \
        "https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz" \
        -o /tmp/macos-sdk.tar.xz \
    && mkdir -p /opt/sdks \
    && tar -xf /tmp/macos-sdk.tar.xz -C /opt/sdks \
    && rm /tmp/macos-sdk.tar.xz
ENV SDKROOT=/opt/sdks/MacOSX11.3.sdk

# ── Rust cross-compilation targets ───────────────────────────────────────────
# Linux
RUN rustup target add \
        x86_64-unknown-linux-gnu \
        i686-unknown-linux-gnu \
        aarch64-unknown-linux-gnu
# Windows (GNU ABI — supported by Zig linker without MinGW install)
RUN rustup target add \
        x86_64-pc-windows-gnu \
        i686-pc-windows-gnu \
        aarch64-pc-windows-gnullvm
# macOS (Zig provides Darwin libc/runtime stubs)
RUN rustup target add \
        x86_64-apple-darwin \
        aarch64-apple-darwin

# ── Entrypoint ────────────────────────────────────────────────────────────────
COPY containers/release/run-release.sh /usr/local/bin/run-release.sh
RUN chmod +x /usr/local/bin/run-release.sh

WORKDIR /workspace
ENTRYPOINT ["/bin/bash", "/usr/local/bin/run-release.sh"]
