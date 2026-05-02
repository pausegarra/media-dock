# How to Install Pullyt

Install Pullyt from the GitHub Release assets for your operating system:

- macOS: `.dmg`
- Windows: `.msi`
- Linux (Debian/Ubuntu): `.deb`

## Important: App Is Not Signed

Pullyt is currently distributed as an **unsigned app**. Your operating system may warn or block it the first time you run it.

## macOS (`.dmg`)

1. Download the `.dmg` file from the latest Release.
2. Open the `.dmg` file.
3. Drag **Media Dock.app** to your **Applications** folder.
4. Before opening the app, run:

```bash
xattr -dr com.apple.quarantine "/Applications/Pullyt.app"
```

5. Open the app from Applications.

If macOS still warns you, open **System Settings -> Privacy & Security** and allow opening the app.

## Windows (`.msi`)

1. Download the `.msi` file from the latest Release.
2. Double-click the installer and follow the setup wizard.
3. If Windows SmartScreen appears, click **More info** -> **Run anyway**.
4. Launch Pullyt from the Start menu after installation.

## Linux (Debian/Ubuntu, `.deb`)

1. Download the `.deb` file from the latest Release.
2. Install it with:

```bash
sudo apt install ./pullyt_<version>_amd64.deb
```

3. Launch Pullyt from your applications menu.

If `apt` reports dependency issues, run:

```bash
sudo apt -f install
```

## First Launch Notes

- On first run, Pullyt may auto-install `yt-dlp`, FFmpeg, and FFprobe to `~/.pullyt/` if they are not already available on your system.
- This can take a short moment depending on your network connection.
