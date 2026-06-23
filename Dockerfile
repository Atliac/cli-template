# syntax=docker/dockerfile:1

# Usage (Podman or Docker — both work):
#   podman build -t cli-template .
#   podman run --rm cli-template
#   podman run --rm cli-template <other-bin>          # run a different binary
#   podman run --rm -it cli-template                  # use -it for interactive CLIs

# Change this to your primary binary name.
ARG DEFAULT_BIN=cli-template

FROM rust:bookworm AS builder

ARG DEFAULT_BIN

RUN rustc --version && cargo --version

WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --workspace && \
    mkdir -p /out && \
    for dir in */; do \
      [ -f "$dir/Cargo.toml" ] || continue; \
      pkg=$(grep '^name' "$dir/Cargo.toml" | head -1 | sed 's/.*= *"\(.*\)".*/\1/'); \
      [ -n "$pkg" ] && [ -f "target/release/$pkg" ] && cp "target/release/$pkg" /out/ || true; \
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

CMD ["/bin/sh", "-c", "exec /usr/local/bin/${DEFAULT_BIN}"]
