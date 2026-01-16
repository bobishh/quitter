# Stage 1: Build Frontend
FROM rust:latest AS frontend-builder
WORKDIR /build

# Install dependencies for Wasm and Trunk
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl protobuf-compiler
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY . .
WORKDIR /build/frontend
RUN trunk build --release

# Stage 2: Build Backend
FROM rust:latest AS backend-builder
WORKDIR /build

# Install protoc for the shared crate
RUN apt-get update && apt-get install -y protobuf-compiler libssl-dev pkg-config

COPY . .
WORKDIR /build/api
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=backend-builder /build/target/release/api /app/server

# Copy frontend assets
COPY --from=frontend-builder /build/frontend/dist /frontend/dist

# Set environment variables
ENV DATABASE_URL=""
ENV PORT=8080

EXPOSE 8080

# We run from /app, so ../frontend/dist (as hardcoded in server) points to /frontend/dist
CMD ["/app/server"]
