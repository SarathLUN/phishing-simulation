FROM rust:1.86 AS builder

WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Build the application
RUN cargo build --release

# Use the same base image for runtime to ensure compatibility
FROM rust:1.86-slim

WORKDIR /app

# Install only the necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/phishing_email .

EXPOSE 80
CMD ["./phishing_email"]
