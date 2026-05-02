# Pullyt

Rust desktop app using [Tauri](https://tauri.app/) backend + Svelte frontend. Downloads media via `yt-dlp`, with FFmpeg auto-installed at startup for cross-platform encoding.

## Build & Run

```bash
cd ui && npm install
cargo build --release
cargo tauri dev
cargo test
cargo check
```

## Architecture

Single module: `modules/downloader/` — Clean Architecture with:
- `domain/` — entities, value objects, ports (traits), errors
- `application/` — use cases
- `infrastructure/` — adapters (yt_dlp, save_dialog, dependencies)
- `presentation/` — Tauri command/event layer

Frontend lives in `ui/` (Svelte + Vite).

Entry point: `src/main.rs` calls `pullyt::modules::downloader::presentation::tauri_app::run()`.

## Dependencies

The app auto-installs FFmpeg/FFprobe and `yt-dlp` to `~/.pullyt/` when not found on PATH.

## Tests

Unit tests live in `tests/unit/` and integration tests in `tests/integration/`. The `tests/unit.rs` and `tests/integration.rs` files use `#[path = "..."]` to include submodules.

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs on macOS, Ubuntu, and Windows. Build order: `cargo check` → `cargo test` → `cargo build --release`.

## Rust Edition

`Cargo.toml` specifies `edition = "2024"`. Ensure your Rust toolchain supports this edition.
