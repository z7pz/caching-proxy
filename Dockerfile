# Use official Rust image as a base
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy project files
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# Use a smaller base image to reduce size
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/cache-proxy /app/cache-proxy

# Expose the port for the proxy
EXPOSE 3000

# Run the application
CMD ["/app/cache-proxy", "--port", "3000", "--origin", "http://example.com", "--cache-ttl", "120"]
