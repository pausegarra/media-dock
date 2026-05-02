<img src="src/modules/downloader/presentation/assets/logo.svg" width="400" alt="Pullyt Logo" style="display:block;margin-left:auto;margin-right:auto;">

# Pullyt

A desktop application with a Rust + Tauri backend and a Svelte frontend for downloading media using `yt-dlp` with FFmpeg for cross-platform encoding.

## Installation

For packaged install instructions on macOS (`.dmg`), Windows (`.exe`), and Linux (`.deb`), see [`HOW_TO_INSTALL.md`](HOW_TO_INSTALL.md).

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
cd ui && npm install
cargo build --release
cargo tauri dev
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
└── presentation/    # Tauri backend commands/events
```

UI frontend: `ui/` (Svelte + Vite).

Entry point: `src/main.rs` calls `pullyt::modules::downloader::presentation::tauri_app::run()`.

## CI/CD

GitHub Actions runs on macOS, Ubuntu, and Windows with the workflow:
`cargo check` → `cargo test` → `cargo build --release`

## Tauri Updater

- Updater plugin is enabled in `tauri.conf.json` and registered in `src/modules/downloader/presentation/tauri_app.rs`.
- Windows release target is NSIS (`.exe`) and updater checks are exposed in-app via a manual `Check for updates` button.
- Configure `plugins.updater.pubkey` and `plugins.updater.endpoints` in `tauri.conf.json` before shipping.

Generate signing keys:

```bash
npm run tauri signer generate -- -w ~/.tauri/pullyt.key
```

Set GitHub repository secrets for release signing:

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

Publish a `latest.json` endpoint that includes per-platform signed artifacts (`.sig`) for:

- `windows-x86_64` (NSIS setup exe)
- `darwin-aarch64`
- `darwin-x86_64`
- `linux-x86_64`

Starter template: `updater/latest.json.template`

## Dependencies

| Tool | Location |
|------|----------|
| FFmpeg/FFprobe | `~/.pullyt/` (auto-installed) |
| yt-dlp | `~/.pullyt/` (auto-installed) |
