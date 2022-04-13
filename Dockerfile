FROM rust:1.59.0-buster as builder
ENV USER root
RUN rustup component add rustfmt
RUN rustup target add wasm32-unknown-unknown
WORKDIR /kustodio
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
COPY src src
RUN mkdir ui
ADD ui/Cargo.toml ui/Cargo.toml
ADD ui/Cargo.lock ui/Cargo.lock
COPY ui/src ui/src
RUN cargo build --release

FROM debian:latest
COPY --from=builder /kustodio/target/release/kustodio /kustodio
ADD example/kustodio-peer-0.toml /example.toml
ENV RUST_LOG=info
CMD ["/kustodio"]
