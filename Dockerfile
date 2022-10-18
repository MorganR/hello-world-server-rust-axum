# syntax=docker/dockerfile:1

### Build the server
FROM rust:bullseye AS serverbuild

WORKDIR /app
COPY . ./

# Compile binary
RUN cargo build -r

### Final image ###
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=serverbuild /app/target/release/hello-world-server-rust-axum ./
ENTRYPOINT ["./hello-world-server-rust-axum"]