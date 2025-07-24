FROM rust:1.88-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates gcc libc6-dev libssl-dev && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/lexoffice-cli /usr/local/bin/lexoffice-cli

ENTRYPOINT ["lexoffice-cli"]