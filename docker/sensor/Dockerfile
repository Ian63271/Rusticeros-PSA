FROM rustlang/rust:nightly-bookworm AS builder

WORKDIR /app

COPY rust/ .

RUN cargo build --release -p sensor

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libzmq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/sensor .

COPY docker/sensor/entrypoint.sh .
RUN chmod +x entrypoint.sh

CMD ["./entrypoint.sh"]
