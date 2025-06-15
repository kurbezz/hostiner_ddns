FROM rust:bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --bin hostinger_ddns


FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y openssl ca-certificates curl jq \
    && rm -rf /var/lib/apt/lists/*

RUN update-ca-certificates

ARG COMMIT_HASH
ENV COMMIT_HASH=${COMMIT_HASH:-unknown}

COPY --from=builder /app/target/release/hostinger_ddns /usr/local/bin
CMD ["/usr/local/bin/hostinger_ddns"]
