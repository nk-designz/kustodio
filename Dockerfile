FROM rust:1.59.0-buster as builder
ENV USER root
RUN apt update && apt install npm protobuf-compiler cmake -y
RUN rustup component add rustfmt
RUN rustup target add wasm32-unknown-unknown
WORKDIR /kustodio
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
COPY src src
COPY proto proto
RUN mkdir ui
ADD ui/Cargo.toml ui/Cargo.toml
ADD ui/Cargo.lock ui/Cargo.lock
COPY ui/src ui/src
RUN mkdir ui/dist
ENV RUSTFLAGS="--cfg tokio_unstable"
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /kustodio/target/release/kustodio /kustodio
ADD example/kustodio-peer-0.toml /example.toml
ENV RUST_LOG=info
CMD ["/kustodio", "server", "example.toml"]
