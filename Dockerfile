FROM rust:1.75.0

WORKDIR /app

COPY . .

RUN cargo install cargo-watch
