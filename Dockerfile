# syntax=docker/dockerfile:1

# Usage (Podman or Docker — both work):
#   podman build -t cli-template .
#   podman run --rm cli-template
#   podman run --rm cli-template <other-bin>          # run a different binary
#   podman run --rm -it cli-template                  # use -it for interactive CLIs

# Change this to your primary binary name.
ARG DEFAULT_BIN=cli-template

FROM --platform=$BUILDPLATFORM rust:bookworm AS builder

ARG TARGETARCH
ARG DEFAULT_BIN

RUN rustc --version && cargo --version

# Install cross-compilation toolchain for non-native targets
RUN if [ "${TARGETARCH}" = "arm64" ]; then \
      dpkg --add-architecture arm64 && \
      apt-get update && \
      apt-get install -y gcc-aarch64-linux-gnu; \
    fi

# Always set the aarch64 linker — harmless on amd64 builds
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

# Determine Rust target triple and install it
RUN RUST_TARGET=$(case "${TARGETARCH}" in \
      amd64) echo "x86_64-unknown-linux-gnu" ;; \
      arm64) echo "aarch64-unknown-linux-gnu" ;; \
      *)     echo "Unsupported: ${TARGETARCH}" >&2; exit 1 ;; \
    esac) && \
    rustup target add "${RUST_TARGET}" && \
    echo "${RUST_TARGET}" > /rust_target

WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    RUST_TARGET=$(cat /rust_target) && \
    cargo build --release --locked --workspace --target "${RUST_TARGET}" && \
    mkdir -p /out && \
    for dir in */; do \
      [ -f "$dir/Cargo.toml" ] || continue; \
      pkg=$(grep '^name' "$dir/Cargo.toml" | head -1 | sed 's/.*= *"\(.*\)".*/\1/'); \
      [ -n "$pkg" ] && [ -f "target/${RUST_TARGET}/release/$pkg" ] && cp "target/${RUST_TARGET}/release/$pkg" /out/ || true; \
    done

# ══════════════════════════════════════════════════════════════════════════════
# Runtime stage — uses the target-architecture native base image.
# ══════════════════════════════════════════════════════════════════════════════
FROM debian:bookworm-slim

ARG DEFAULT_BIN
ENV DEFAULT_BIN=$DEFAULT_BIN

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd --no-create-home --shell /sbin/nologin appuser

COPY --from=builder /out/ /usr/local/bin/

WORKDIR /app

USER appuser

CMD exec /usr/local/bin/$DEFAULT_BIN
