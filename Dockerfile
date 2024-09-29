# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Install wasm-pack
RUN cargo install wasm-pack

# Build the project using wasm-pack
RUN wasm-pack build --target web

# Use a Node.js image to serve the built files
FROM node:alpine

# Set the working directory in the container
WORKDIR /app

# Install http-server globally
RUN npm install -g http-server

# Copy the generated wasm files to the working directory
COPY --from=builder /app/pkg /app/pkg

# Copy the index.html file to the working directory
COPY --from=builder /app/index.html /app/index.html

# Expose port 8080 to the outside world
EXPOSE 8080

# Start http-server when the container launches
CMD ["http-server", "-p", "8080", ".", "-g"]