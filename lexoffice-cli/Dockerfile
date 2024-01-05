FROM rust:1.74-bookworm AS builder
WORKDIR /usr/src/lexoffice-cli
COPY . .
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres
RUN unset DATABASE_URL && \
    cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates gcc libc6-dev libssl-dev postgresql && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/lexoffice-cli /usr/local/bin/lexoffice-cli
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /usr/src/lexoffice-cli/migrations /usr/src/lexoffice-cli/migrations
CMD ["lexoffice-cli"]
