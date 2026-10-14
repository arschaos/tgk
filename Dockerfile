# Build
FROM rust:1.85-slim AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Run
FROM debian:bookworm-slim

RUN useradd --system --create-home tgk

COPY --from=builder /app/target/release/tgk /usr/local/bin/tgk

ENTRYPOINT [ "tgk" ]