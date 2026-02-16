# ---------- Stage 1: Build ----------
FROM rust:latest as builder

WORKDIR /app

# Copiamos todo el workspace
COPY . .

# Compilamos coordinator y worker en modo release
RUN cargo build --release -p coordinator
RUN cargo build --release -p worker

# ---------- Stage 2: Runtime ----------
FROM debian:bookworm-slim

WORKDIR /app

# Copiamos los binarios compilados
COPY --from=builder /app/target/release/coordinator .
COPY --from=builder /app/target/release/worker .

# Por defecto no ejecuta nada (lo define docker-compose)

