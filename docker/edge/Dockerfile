FROM rustlang/rust:nightly-bookworm AS builder

WORKDIR /app

COPY rust/ .


RUN cargo build --release -p edge

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libzmq5 \
    iproute2 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/edge .

COPY docker/edge/entrypoint.sh .
RUN chmod +x entrypoint.sh

CMD ["./entrypoint.sh"]
