# Stage 1: Build the Rust app
FROM rust:1.91 AS builder

WORKDIR /usr/src/app

# Copy only the necessary files for caching
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the rest of the source code
COPY . .

# Build the release binary
RUN cargo build --release

# Stage 2: Copy the binary to a minimal image
FROM debian:bookworm-slim

# Install CA certificates for HTTPS support (if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/log_scanner /usr/local/bin/

# Create necessary directories for output
RUN mkdir -p /app/output
VOLUME /app/output

WORKDIR /app

# Expose the port
EXPOSE 8080

# Run the server
ENTRYPOINT ["log_scanner", "server"]