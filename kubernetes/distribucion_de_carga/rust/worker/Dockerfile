# Fase de compilación
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Fase de ejecución (Imagen ligera)
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/worker /usr/local/bin/worker
EXPOSE 3000
CMD ["worker"]
