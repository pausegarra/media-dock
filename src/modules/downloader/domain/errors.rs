use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum DownloaderError {
    #[error("invalid YouTube URL")]
    InvalidUrl,
    #[error("yt-dlp not available")]
    YtDlpUnavailable,
    #[error("ffmpeg bundle not found")]
    FfmpegUnavailable,
    #[error("ffprobe bundle not found")]
    FfprobeUnavailable,
    #[error("download canceled")]
    SaveCanceled,
    #[error("process failed: {0}")]
    ProcessFailed(String),
}
