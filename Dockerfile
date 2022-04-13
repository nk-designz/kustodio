FROM rust:1.59.0-buster as builder
ENV USER root
RUN rustup component add rustfmt
WORKDIR /kustodio
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
COPY src src
RUN mkdir ui
ADD ui/Cargo.toml ui/Cargo.toml
ADD ui/Cargo.lock ui/Cargo.lock
COPY ui/src ui/src
RUN cargo build --release

FROM scratch
COPY --from=builder /kustodio/target/release/kustodio /kustodio
CMD ["/kustodio"]
