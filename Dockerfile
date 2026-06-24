# syntax=docker/dockerfile:1

# Usage (Podman or Docker — both work):
#   podman build -t cli-template .
#   podman run --rm cli-template
#   podman run --rm cli-template --help               # pass flags to the binary
#   podman run --rm --entrypoint <other-bin> cli-template  # run a different binary
#   podman run --rm -it cli-template                  # use -it for interactive CLIs

FROM rust:bookworm AS builder

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

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd --no-create-home --shell /sbin/nologin appuser

COPY --from=builder /out/ /usr/local/bin/

WORKDIR /app

USER appuser

# Change this to your primary binary name.
ENTRYPOINT ["cli-template"]
