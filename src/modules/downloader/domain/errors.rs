use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum DownloaderError {
    #[error("invalid YouTube URL")]
    InvalidUrl,
    #[error("yt-dlp not available")]
    YtDlpUnavailable,
    #[error("ffmpeg not available")]
    FfmpegUnavailable,
    #[error("ffprobe not available")]
    FfprobeUnavailable,
    #[error("download canceled")]
    SaveCanceled,
    #[error("process failed: {0}")]
    ProcessFailed(String),
    #[error("release check failed: {0}")]
    ReleaseCheckFailed(String),
}
