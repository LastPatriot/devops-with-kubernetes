FROM rust:1 AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/random-string /usr/local/bin/random-string
CMD ["random-string"]