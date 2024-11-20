FROM rust:slim

# Install neded components
RUN rustup component add clippy rustfmt && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-pack

# Install git & gpg
RUN apt update -y && \
    apt install -y git gpg