FROM rust:slim

# Install meeded components
RUN rustup component add clippy rustfmt && \
    rustup target add wasm32-unknown-unknown && \
    apt update -y && \
    apt install -y git gpg && \
    cargo install wasm-pack

# Generate Cargo.toml
RUN cargo generate-lockfile