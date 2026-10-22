FROM rust:1 AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/ping_pong /usr/local/bin/ping_pong
CMD ["ping_pong"]