<img src="src/modules/downloader/presentation/assets/logo.svg" width="400" alt="Pullyt Logo" style="display:block;margin-left:auto;margin-right:auto;">

# Pullyt

A Rust desktop application for downloading media using `yt-dlp` with FFmpeg for cross-platform encoding.

## Installation

For packaged install instructions on macOS (`.dmg`), Windows (`.msi`), and Linux (`.deb`), see [`HOW_TO_INSTALL.md`](HOW_TO_INSTALL.md).

## Features

- Download media from various video platforms via `yt-dlp`
- Auto-installed FFmpeg for audio/video encoding
- Cross-platform support (macOS, Linux, Windows)
- Clean Architecture design

## Requirements

- Rust toolchain supporting **edition 2024**
- FFmpeg/FFprobe (auto-installed to `~/.pullyt/` if not on PATH)
- `yt-dlp` (auto-installed to `~/.pullyt/` if not on PATH)

## Build & Run

```bash
cargo build --release
cargo run --release
```

## Testing

```bash
cargo test
cargo check
```

## Icon & Packaging Assets

Source icon: `src/modules/downloader/presentation/assets/icon.png`

Generate platform icon assets:
```bash
./scripts/generate-icons.sh
```

Outputs:
- macOS: `assets/icons/macos/AppIcon.icns`
- Windows: `assets/icons/windows/app.ico`
- Linux: `assets/icons/linux/hicolor/*/apps/pullyt.png`

## Architecture

Single module: `modules/downloader/` following Clean Architecture:

```
modules/downloader/
├── domain/          # entities, value objects, ports (traits), errors
├── application/     # use cases
├── infrastructure/  # adapters (yt_dlp, save_dialog, dependencies)
└── presentation/    # iced UI
```

Entry point: `src/main.rs` calls `pullyt::modules::downloader::presentation::app::run()`.

## CI/CD

GitHub Actions runs on macOS, Ubuntu, and Windows with the workflow:
`cargo check` → `cargo test` → `cargo build --release`

## Dependencies

| Tool | Location |
|------|----------|
| FFmpeg/FFprobe | `~/.pullyt/` (auto-installed) |
| yt-dlp | `~/.pullyt/` (auto-installed) |
