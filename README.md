<img src="src/modules/downloader/presentation/assets/logo.svg" width="400" alt="Media Dock Logo">

# Media Dock

A Rust desktop application for downloading media using `yt-dlp` with FFmpeg for cross-platform encoding.

## Features

- Download media from various video platforms via `yt-dlp`
- Auto-installed FFmpeg for audio/video encoding
- Cross-platform support (macOS, Linux, Windows)
- Clean Architecture design

## Requirements

- Rust toolchain supporting **edition 2024**
- FFmpeg/FFprobe (auto-installed to `~/.media-dock/bin/` if not on PATH)
- `yt-dlp` (auto-installed to `~/.media-dock/bin/` if not on PATH)

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
- Linux: `assets/icons/linux/hicolor/*/apps/media-dock.png`

## Architecture

Single module: `modules/downloader/` following Clean Architecture:

```
modules/downloader/
├── domain/          # entities, value objects, ports (traits), errors
├── application/     # use cases
├── infrastructure/  # adapters (yt_dlp, save_dialog, dependencies)
└── presentation/    # iced UI
```

Entry point: `src/main.rs` calls `media_dock::modules::downloader::presentation::app::run()`.

## CI/CD

GitHub Actions runs on macOS, Ubuntu, and Windows with the workflow:
`cargo check` → `cargo test` → `cargo build --release`

## Dependencies

| Tool | Location |
|------|----------|
| FFmpeg/FFprobe | `~/.media-dock/bin/` (auto-installed) |
| yt-dlp | `~/.media-dock/bin/` (auto-installed) |
