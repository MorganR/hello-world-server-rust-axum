# syntax=docker/dockerfile:1

### Build the server
FROM rust:latest AS serverbuild

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . ./

# Compile binary
RUN cargo build -r --target=x86_64-unknown-linux-musl

### Final image
FROM alpine:latest
COPY --from=serverbuild /app/target/x86_64-unknown-linux-musl/release/hello-world-server-rust-axum /app/
ENTRYPOINT ["/app/hello-world-server-rust-axum"]
