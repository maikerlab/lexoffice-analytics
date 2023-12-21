FROM rust:1.74-bookworm AS builder
WORKDIR /usr/src/lexoffice-cli
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates gcc libc6-dev libssl-dev postgresql && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/lexoffice-cli /usr/local/bin/lexoffice-cli
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
CMD ["lexoffice-cli"]
