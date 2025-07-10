FROM rust:1.82 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app
COPY --from=builder /app/target/release/webservice_rust /app/webservice_rust
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

EXPOSE 8080
CMD ["./webservice_rust"]
