# Fase de compilación
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Fase de ejecución
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/master /usr/local/bin/master
CMD ["master"]
