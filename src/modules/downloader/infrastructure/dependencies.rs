use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::modules::downloader::domain::errors::DownloaderError;
use crate::modules::downloader::domain::ports::DependencyPort;

pub struct SystemDependencies;

impl DependencyPort for SystemDependencies {
    fn ensure_yt_dlp(&self) -> Result<String, DownloaderError> {
        if command_works("yt-dlp") {
            return Ok("yt-dlp".to_string());
        }
        if local_yt_dlp_path().exists() {
            return Ok(local_yt_dlp_path().display().to_string());
        }
        install_local_yt_dlp()?;
        if command_works("yt-dlp") {
            Ok("yt-dlp".to_string())
        } else if local_yt_dlp_path().exists() {
            Ok(local_yt_dlp_path().display().to_string())
        } else {
            Err(DownloaderError::YtDlpUnavailable)
        }
    }

    fn ensure_ffmpeg(&self) -> Result<String, DownloaderError> {
        if command_works("ffmpeg") {
            return Ok("ffmpeg".to_string());
        }

        let bundled = bundled_ffmpeg_path();
        if bundled.exists() {
            return Ok(bundled.display().to_string());
        }

        Err(DownloaderError::FfmpegUnavailable)
    }

    fn ensure_ffprobe(&self) -> Result<String, DownloaderError> {
        if command_works("ffprobe") {
            return Ok("ffprobe".to_string());
        }

        let bundled = bundled_ffprobe_path();
        if bundled.exists() {
            return Ok(bundled.display().to_string());
        }

        Err(DownloaderError::FfprobeUnavailable)
    }
}

pub fn yt_dlp_command() -> String {
    if command_works("yt-dlp") {
        "yt-dlp".to_string()
    } else {
        local_yt_dlp_path().display().to_string()
    }
}

fn command_works(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn install_local_yt_dlp() -> Result<(), DownloaderError> {
    let bin_dir = local_bin_dir();
    fs::create_dir_all(&bin_dir).map_err(|e| DownloaderError::ProcessFailed(e.to_string()))?;
    let target = local_yt_dlp_path();

    #[cfg(target_os = "windows")]
    {
        let script = format!(
            "iwr https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe -OutFile '{}'",
            target.display()
        );
        let status = Command::new("powershell")
            .arg("-Command")
            .arg(script)
            .status()
            .map_err(|e| DownloaderError::ProcessFailed(e.to_string()))?;
        if !status.success() {
            return Err(DownloaderError::YtDlpUnavailable);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let status = Command::new("curl")
            .args([
                "-L",
                "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
                "-o",
            ])
            .arg(target.display().to_string())
            .status()
            .map_err(|e| DownloaderError::ProcessFailed(e.to_string()))?;
        if !status.success() {
            return Err(DownloaderError::YtDlpUnavailable);
        }
        let _ = Command::new("chmod").args(["+x", &target.display().to_string()]).status();
    }

    Ok(())
}

fn local_bin_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Path::new(&home).join(".media-dock").join("bin")
}

fn local_yt_dlp_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return local_bin_dir().join("yt-dlp.exe");
    }
    #[cfg(not(target_os = "windows"))]
    {
        local_bin_dir().join("yt-dlp")
    }
}

fn bundled_ffmpeg_path() -> PathBuf {
    for candidate in bundled_ffmpeg_candidates() {
        if candidate.exists() {
            return candidate;
        }
    }

    #[cfg(target_os = "windows")]
    {
        Path::new("bin").join("windows").join("ffmpeg.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        Path::new("bin").join("macos").join("ffmpeg")
    }
}

fn bundled_ffmpeg_candidates() -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let arch = std::env::consts::ARCH;
        let arch_name = match arch {
            "aarch64" => "arm64",
            "x86_64" => "x64",
            _ => "arm64",
        };
        return vec![
            Path::new("bin").join("macos").join("ffmpeg"),
            Path::new("bin").join(format!("ffmpeg-darwin-{arch_name}")),
        ];
    }

    #[cfg(target_os = "linux")]
    {
        let arch = std::env::consts::ARCH;
        let arch_name = match arch {
            "aarch64" => "arm64",
            "x86_64" => "x64",
            "x86" | "i686" => "ia32",
            "arm" => "arm",
            _ => "x64",
        };
        return vec![
            Path::new("bin").join("linux").join("ffmpeg"),
            Path::new("bin").join(format!("ffmpeg-linux-{arch_name}")),
        ];
    }

    #[cfg(target_os = "windows")]
    {
        vec![
            Path::new("bin").join("windows").join("ffmpeg.exe"),
            Path::new("bin").join("ffmpeg-win32-x64"),
        ]
    }
}

fn bundled_ffprobe_path() -> PathBuf {
    for candidate in bundled_ffprobe_candidates() {
        if candidate.exists() {
            return candidate;
        }
    }

    #[cfg(target_os = "windows")]
    {
        Path::new("bin").join("windows").join("ffprobe.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        Path::new("bin").join("macos").join("ffprobe")
    }
}

fn bundled_ffprobe_candidates() -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let arch = std::env::consts::ARCH;
        let arch_name = match arch {
            "aarch64" => "arm64",
            "x86_64" => "x64",
            _ => "arm64",
        };
        return vec![
            Path::new("bin").join("macos").join("ffprobe"),
            Path::new("bin").join(format!("ffprobe-darwin-{arch_name}")),
        ];
    }

    #[cfg(target_os = "linux")]
    {
        let arch = std::env::consts::ARCH;
        let arch_name = match arch {
            "aarch64" => "arm64",
            "x86_64" => "x64",
            "x86" | "i686" => "ia32",
            "arm" => "arm",
            _ => "x64",
        };
        return vec![
            Path::new("bin").join("linux").join("ffprobe"),
            Path::new("bin").join(format!("ffprobe-linux-{arch_name}")),
        ];
    }

    #[cfg(target_os = "windows")]
    {
        vec![
            Path::new("bin").join("windows").join("ffprobe.exe"),
            Path::new("bin").join("ffprobe-win32-x64"),
        ]
    }
}
