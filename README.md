# Quitter (Alcoholics Audacious)

A habit avoidance and abstinence tracker web app.

## Overview

## Prerequisites
- **Rust:** Latest stable version.
- **PostgreSQL:** Database server.
- **Trunk:** For building the Leptos frontend (`cargo install trunk`).
- **SQLx CLI:** For database migrations (`cargo install sqlx-cli`).
- **Protobuf Compiler (`protoc`):** Required for compiling the shared proto files.

## Setup
1.  **Database:**
    Create a PostgreSQL database (e.g., `addict`).
    ```bash
    export DATABASE_URL="postgres://postgres:password@localhost/addict"
    ```

2.  **Migrations:**
    Run migrations to set up the schema.
    ```bash
    cd api
    sqlx migrate run
    ```

## Running the Application

### Backend

The backend serves the API and the static frontend files.

```bash
# In the api/ directory
cargo run
```

The server defaults to port `8080`.

### Frontend

For development with hot-reloading:

```bash
# In the frontend/ directory
trunk serve
```

To build for production (files will be output to `frontend/dist`, which the API server serves):

```bash
# In the frontend/ directory
trunk build --release
```

## Project Structure

- `api/`: Backend server (Axum).
- `frontend/`: Frontend application (Leptos).
- `shared/`: Shared library and Protocol Buffer definitions.
