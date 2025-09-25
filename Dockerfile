FROM rust:1.90-slim

WORKDIR /app
COPY . .

VOLUME /target /app/targets

ENV SQLX_OFFLINE=true
RUN cargo build --release
ENTRYPOINT ["./target/release/zero2prod"]
