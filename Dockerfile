# Stage 1: Builder
FROM rust:1.80-slim-bookworm as builder

WORKDIR /usr/src/rvoip

# Install build dependencies
# - pkg-config, libssl-dev: for openssl-sys
# - libsqlite3-dev: for sqlx with sqlite
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY examples ./examples

# Build Release
# Note: We specifically build the main binary 'rvoip'
RUN cargo build --release -p rvoip

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    openssl \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create user for security
RUN useradd -ms /bin/bash rvoip

# Create directory structure
RUN mkdir -p /app/data /app/certs

# Copy binary from builder
COPY --from=builder /usr/src/rvoip/target/release/rvoip /usr/local/bin/rvoip

# Set ownership
RUN chown -R rvoip:rvoip /app

# Switch to non-root user
USER rvoip

# Expose ports
# 5060: SIP UDP/TCP
# 5061: SIPS TCP
# 8080: WS
# 8443: WSS
# 10000-20000: RTP (Range usually configured in docker-compose network_mode: host)
EXPOSE 5060/udp 5060/tcp 5061/tcp 8080/tcp 8443/tcp

# Runtime environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:/app/data/rvoip.db

# Entrypoint
CMD ["rvoip"]
