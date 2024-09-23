FROM rust:latest

COPY ./ ./
RUN rustup target add wasm32-unknown-unknown && rustup component add clippy
RUN cargo generate-lockfile

# Build
RUN cargo build && cargo build --release
# Build (Wasm)
RUN cargo build --target wasm32-unknown-unknown && cargo build --target wasm32-unknown-unknown --release
# Tests
RUN cargo test
# Code analysis
RUN cargo clippy
