# Base image with all shared tools and dependencies
# We cache these tools so we don't install them on every build
FROM rust:latest AS base
WORKDIR /app
# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev protobuf-compiler
# Install build tools
RUN cargo install cargo-chef trunk
# Add WASM target for frontend
RUN rustup target add wasm32-unknown-unknown

# Stage 1: Planner - Computes the lockfile and recipe
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cacher - Builds dependencies based on recipe
FROM base AS cacher
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies for Backend (Native)
RUN cargo chef cook --release --recipe-path recipe.json --package api
# Build dependencies for Frontend (WASM)
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json --package addict_tracker

# Stage 3: Backend Builder
FROM base AS backend-builder
COPY . .
# Copy cached dependencies from cacher
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# Build the actual API binary
RUN cargo build --release --bin api

# Stage 4: Frontend Builder
FROM base AS frontend-builder
COPY . .
# Copy cached dependencies from cacher
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
WORKDIR /app/frontend
# Build the frontend assets
RUN trunk build --release

# Stage 5: Runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from backend builder
COPY --from=backend-builder /app/target/release/api /app/server

# Copy frontend assets from frontend builder
COPY --from=frontend-builder /app/frontend/dist /frontend/dist

# Set environment variables
ENV DATABASE_URL=""
ENV PORT=8080
ENV FRONTEND_DIST="/frontend/dist"

EXPOSE 8080

# Run the server
CMD ["/app/server"]