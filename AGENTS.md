# Addict Tracker (Winamp Edition)

## Project Overview
A Rust + Leptos WebAssembly application for tracking substance usage and abstinence. It features a retro "Winamp-style" aesthetic and a dual-mode tracking system.

## Core Concepts
The domain model is strictly separated into:
1.  **Habit (Definition):** Static configuration (e.g., "Weed", "Alcohol"). Contains:
    *   Name, Icon, Color
    *   Frequency (for calculation)
    *   Unit Name (e.g., "Joint")
2.  **Tracker (State):** The active tracking instance.
    *   **Abstinence Mode:** Tracks time *since* a specific date.
    *   **Usage Mode:** Logs specific usage events (timestamps).
    *   Links to a `Habit` via `habit_id`.

## Tech Stack
*   **Language:** Rust (2024 edition)
*   **Framework:** Leptos (Signals, Components)
*   **Bundler:** Trunk
*   **Storage:** Browser LocalStorage (via `gloo-storage`)
*   **Styling:** Raw CSS (Dark/Terminal/Winamp theme)

## Development Guidelines (TDD)
**Strict Test-Driven Development (TDD) is enforced.**

1.  **Red:** Write a failing test for the new feature or logic.
2.  **Green:** Write the minimal code to pass the test.
3.  **Refactor:** Improve code quality while ensuring tests stay green.

### Testing Strategy
*   **Domain Logic:** Unit tests in `src/model.rs` (or `tests/`) for calculations and state transitions.
*   **WASM/Browser Integration:** Use `wasm-bindgen-test` for components or storage logic requiring a JS environment.

## Running Tests
```bash
# Pure logic tests
cargo test

# WASM-specific tests (requires node or headless browser)
wasm-pack test --headless --firefox 
# OR via cargo if configured
cargo test --target wasm32-unknown-unknown
```
