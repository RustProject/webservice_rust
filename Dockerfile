# Stage 1: Build
FROM rust:1.72 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Run
FROM debian:buster-slim

WORKDIR /app
COPY --from=builder /app/target/release/webservice_rust /app/webservice_rust
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

EXPOSE 8080
CMD ["./webservice_rust"]
