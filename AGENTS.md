# Pullryn

Rust desktop app using [iced](https://iced.rs/) GUI framework. Downloads media via `yt-dlp`, with FFmpeg auto-installed at startup for cross-platform encoding.

## Build & Run

```bash
cargo build --release
cargo run --release
cargo test
cargo check
```

## Architecture

Single module: `modules/downloader/` — Clean Architecture with:
- `domain/` — entities, value objects, ports (traits), errors
- `application/` — use cases
- `infrastructure/` — adapters (yt_dlp, save_dialog, dependencies)
- `presentation/` — iced UI

Entry point: `src/main.rs` calls `pullryn::modules::downloader::presentation::app::run()`.

## Dependencies

The app auto-installs FFmpeg/FFprobe and `yt-dlp` to `~/.pullryn/` when not found on PATH.

## Tests

Unit tests live in `tests/unit/` and integration tests in `tests/integration/`. The `tests/unit.rs` and `tests/integration.rs` files use `#[path = "..."]` to include submodules.

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs on macOS, Ubuntu, and Windows. Build order: `cargo check` → `cargo test` → `cargo build --release`.

## Rust Edition

`Cargo.toml` specifies `edition = "2024"`. Ensure your Rust toolchain supports this edition.
