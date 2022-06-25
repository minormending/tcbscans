FROM rust:1.61-slim as builder

WORKDIR /usr/src/tcbscans
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/tcbscans /usr/local/bin/tcbscans

ENTRYPOINT ["tcbscans"]
