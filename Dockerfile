FROM rust:slim as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /usr/src/tcbscans
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest
COPY --from=builder /usr/src/tcbscans/target/x86_64-unknown-linux-musl/release/tcbscans /usr/local/bin/tcbscans

ENTRYPOINT ["tcbscans"]
