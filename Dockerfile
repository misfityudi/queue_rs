# ---- Build Stage ----
FROM rust:latest as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for better caching
COPY Cargo.toml ./

# Copy source code and migrations
COPY src ./src
COPY migrations ./migrations
COPY .env* ./

# Set a placeholder DATABASE_URL to prevent sqlx from trying to connect
ENV DATABASE_URL="postgresql://placeholder:placeholder@localhost:5432/placeholder"

# Build the application without database connectivity
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/project-5_api /app/app

# Copy migrations and config files
COPY migrations ./migrations
COPY --from=builder /app/.env* ./

# Create non-root user
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /app

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 8080

USER appuser

CMD ["/app/app"]
