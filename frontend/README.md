# Addict Tracker (Winamp Edition)

A Rust + Leptos + WebAssembly habit tracker with a retro Winamp aesthetic.

## Features
- **Abstinence Tracking:** Tracks time since you quit.
- **Visualizer:** Converts "time sober" into "units avoided" (e.g., piles of cigarettes/joints/beers) to visualize your progress.
- **Local Storage:** Data is saved in your browser.
- **Theming:** Create custom habits with specific colors and icons.

## How to Run

1.  Ensure you have `trunk` installed:
    ```bash
    cargo install trunk
    ```
2.  Run the development server:
    ```bash
    cd addict_tracker
    trunk serve --open
    ```
3.  The app will open in your default browser at `http://127.0.0.1:8080`.

## Tech Stack
-   **Leptos:** Reactive web framework.
-   **Trunk:** WASM bundler.
-   **Styling:** Raw CSS (Retro/Terminal style).
