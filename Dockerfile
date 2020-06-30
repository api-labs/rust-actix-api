FROM rust:1.44

WORKDIR /usr/src/rust-actix-api
COPY . .

RUN cargo build --release
