FROM rust:1.82 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

CMD ["./target/release/webservice_rust"]
