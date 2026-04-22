# ---------- Build stage ----------
FROM rustlang/rust:nightly-bookworm AS builder

WORKDIR /app

# Copiamos workspace desde /rust
COPY rust/ .

RUN cargo build --release -p coordinator

# ---------- Runtime ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libzmq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/coordinator .

# (opcional) usar entrypoint
COPY docker/coordinator/entrypoint.sh .
RUN chmod +x entrypoint.sh

CMD ["./entrypoint.sh"]
