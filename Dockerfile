FROM rust:1.82-bookworm AS builder
WORKDIR /usr/src/lexoffice-analytics
COPY . .
RUN cargo install --path lexoffice-cli

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates gcc libc6-dev libssl-dev && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/lexoffice-cli /usr/local/bin/lexoffice-cli

ENTRYPOINT ["lexoffice-cli"]