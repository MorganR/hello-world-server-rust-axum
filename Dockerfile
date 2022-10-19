# syntax=docker/dockerfile:1

### Build the server
FROM rust:bullseye AS serverbuild

WORKDIR /app
COPY . ./

# Compile binary
RUN cargo build -r

ENTRYPOINT ["/app/target/release/hello-world-server-rust-axum"]