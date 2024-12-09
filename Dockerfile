FROM rust:1.83 AS builder
WORKDIR /authorization
COPY . .
RUN cargo build --release --bin authorization

FROM debian:bookworm-slim
WORKDIR /authorization
COPY --from=builder /authorization/target/release/authorization /usr/local/bin
CMD ["authorization"]

