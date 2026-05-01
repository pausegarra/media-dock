use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::modules::downloader::domain::entities::{
    AudioQuality, DownloadMode, DownloadPreset, DownloadProgress, DownloadRequest, VideoQuality,
};
use crate::modules::downloader::domain::errors::DownloaderError;
use crate::modules::downloader::domain::ports::DownloadPort;

use super::dependencies::yt_dlp_command;

pub struct YtDlpAdapter;

impl DownloadPort for YtDlpAdapter {
    fn run_download(
        &self,
        request: &DownloadRequest,
        ffmpeg_path: &str,
        on_progress: &mut dyn FnMut(DownloadProgress),
    ) -> Result<(), DownloaderError> {
        on_progress(DownloadProgress {
            fraction: 0.05,
            message: "Starting download".to_string(),
        });

        let mut cmd = Command::new(yt_dlp_command());
        cmd.arg("--newline")
            .arg("--progress")
            .arg("--ffmpeg-location")
            .arg(ffmpeg_path)
            .arg("-o")
            .arg(&request.output_path)
            .arg(&request.url)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        match request.mode {
            DownloadMode::AudioOnlyMp3 => {
                cmd.args([
                    "-x",
                    "--audio-format",
                    "mp3",
                    "--audio-quality",
                    audio_quality_value(request.audio_quality),
                ]);
            }
            DownloadMode::VideoWithAudio => {
                cmd.arg("-f").arg(video_audio_format(
                    request.video_quality,
                    request.audio_quality,
                    request.preset,
                ));
                cmd.args(["--merge-output-format", merge_format(request.preset)]);
            }
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| DownloaderError::ProcessFailed(e.to_string()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| DownloaderError::ProcessFailed("missing stdout".to_string()))?;
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let line = line.map_err(|e| DownloaderError::ProcessFailed(e.to_string()))?;
            if let Some(p) = parse_progress(&line) {
                on_progress(DownloadProgress {
                    fraction: p,
                    message: format!("Downloading {:.0}%", p * 100.0),
                });
            }
        }

        let status = child
            .wait()
            .map_err(|e| DownloaderError::ProcessFailed(e.to_string()))?;
        if status.success() {
            on_progress(DownloadProgress {
                fraction: 1.0,
                message: "Finished".to_string(),
            });
            Ok(())
        } else {
            Err(DownloaderError::ProcessFailed(format!(
                "yt-dlp exited with {status}"
            )))
        }
    }
}

fn video_audio_format(video: VideoQuality, audio: AudioQuality, preset: DownloadPreset) -> String {
    let v = match video {
        VideoQuality::Best => "bestvideo",
        VideoQuality::P1080 => "bestvideo[height<=1080]",
        VideoQuality::P720 => "bestvideo[height<=720]",
        VideoQuality::P480 => "bestvideo[height<=480]",
    };
    let a = match audio {
        AudioQuality::Best => "bestaudio",
        AudioQuality::K320 => "bestaudio[abr<=320]",
        AudioQuality::K192 => "bestaudio[abr<=192]",
        AudioQuality::K128 => "bestaudio[abr<=128]",
    };

    match preset {
        DownloadPreset::Compatibility => {
            let v_compat = match video {
                VideoQuality::Best => "bestvideo[ext=mp4][vcodec^=avc1]",
                VideoQuality::P1080 => "bestvideo[ext=mp4][vcodec^=avc1][height<=1080]",
                VideoQuality::P720 => "bestvideo[ext=mp4][vcodec^=avc1][height<=720]",
                VideoQuality::P480 => "bestvideo[ext=mp4][vcodec^=avc1][height<=480]",
            };
            let a_compat = match audio {
                AudioQuality::Best => "bestaudio[ext=m4a]",
                AudioQuality::K320 => "bestaudio[ext=m4a][abr<=320]",
                AudioQuality::K192 => "bestaudio[ext=m4a][abr<=192]",
                AudioQuality::K128 => "bestaudio[ext=m4a][abr<=128]",
            };
            format!("{v_compat}+{a_compat}/{v}+{a}/best[ext=mp4]/best")
        }
        DownloadPreset::MaxQuality => {
            format!("{v}+{a}/best")
        }
    }
}

fn merge_format(preset: DownloadPreset) -> &'static str {
    match preset {
        DownloadPreset::Compatibility => "mp4",
        DownloadPreset::MaxQuality => "mkv",
    }
}

fn audio_quality_value(audio: AudioQuality) -> &'static str {
    match audio {
        AudioQuality::Best => "0",
        AudioQuality::K320 => "320K",
        AudioQuality::K192 => "192K",
        AudioQuality::K128 => "128K",
    }
}

fn parse_progress(line: &str) -> Option<f32> {
    let marker = "[download]";
    if !line.contains(marker) || !line.contains('%') {
        return None;
    }
    let percent_idx = line.find('%')?;
    let prefix = &line[..percent_idx];
    let num = prefix.split_whitespace().last()?;
    let val: f32 = num.parse().ok()?;
    Some((val / 100.0).clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::parse_progress;

    #[test]
    fn parses_percentage_line() {
        let p = parse_progress("[download]  42.4% of 12.01MiB at 2.11MiB/s ETA 00:04").unwrap();
        assert!(p > 0.42 && p < 0.43);
    }
}
