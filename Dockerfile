# Stellarr Dockerfile
# Build and run without needing local Rust/VS Build Tools

FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install dependencies for SQLx and OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first (for caching)
COPY Cargo.toml Cargo.lock ./
COPY crates/stellarr-core/Cargo.toml crates/stellarr-core/
COPY crates/stellarr-db/Cargo.toml crates/stellarr-db/
COPY crates/stellarr-api/Cargo.toml crates/stellarr-api/
COPY crates/stellarr-providers/Cargo.toml crates/stellarr-providers/
COPY crates/stellarr-web/Cargo.toml crates/stellarr-web/
COPY stellarr/Cargo.toml stellarr/

# Create dummy files to build dependencies
RUN mkdir -p crates/stellarr-core/src && echo "pub fn dummy() {}" > crates/stellarr-core/src/lib.rs && \
    mkdir -p crates/stellarr-db/src && echo "pub fn dummy() {}" > crates/stellarr-db/src/lib.rs && \
    mkdir -p crates/stellarr-api/src && echo "pub fn dummy() {}" > crates/stellarr-api/src/lib.rs && \
    mkdir -p crates/stellarr-providers/src && echo "pub fn dummy() {}" > crates/stellarr-providers/src/lib.rs && \
    mkdir -p crates/stellarr-web/src && echo "pub fn dummy() {}" > crates/stellarr-web/src/lib.rs && \
    mkdir -p stellarr/src && echo "fn main() {}" > stellarr/src/main.rs

# Build dependencies (this layer gets cached)
RUN cargo build --release 2>/dev/null || true

# Copy actual source code
COPY . .

# Touch files to invalidate cache and rebuild
RUN touch crates/*/src/*.rs stellarr/src/*.rs

# Build the actual application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/stellarr /app/stellarr

# Copy migrations
COPY --from=builder /app/crates/stellarr-db/migrations /app/migrations

# Create data directory
RUN mkdir -p /app/data

# Environment
ENV STELLARR_SERVER_HOST=0.0.0.0
ENV STELLARR_SERVER_PORT=8080
ENV STELLARR_DATABASE_URL=sqlite:///app/data/stellarr.db

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost:8080/api/health || exit 1

CMD ["/app/stellarr", "serve"]
