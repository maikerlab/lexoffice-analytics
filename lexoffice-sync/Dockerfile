FROM rust:1.74-bookworm AS builder
WORKDIR /usr/src/lexoffice-cli
COPY . .
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel migrate run

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates gcc libc6-dev libssl-dev && \
    rm -rf /var/lib/apt/lists/*
RUN diesel migration run
COPY --from=builder /usr/local/cargo/bin/lexoffice-cli /usr/local/bin/lexoffice-cli
CMD ["lexoffice-cli sync"]
