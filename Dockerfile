FROM rust:slim

# Install needed components
RUN rustup target add wasm32-unknown-unknown && \
    rustup component add clippy

# Prepare the working directory
WORKDIR /usr/src/lib
COPY ./ ./
RUN cargo generate-lockfile

# Build (default target)
RUN cargo build && \
    cargo build --release

# Build (Wasm)
RUN cargo build --target wasm32-unknown-unknown && \
    cargo build --target wasm32-unknown-unknown --release

# Generate documentation
RUN cargo doc --no-deps

# Tests
RUN cargo test

# Code analysis
RUN cargo clippy -- -D warnings
