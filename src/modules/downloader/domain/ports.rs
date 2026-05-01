use super::entities::{DownloadMode, DownloadPreset, DownloadProgress, DownloadRequest};
use super::errors::DownloaderError;

pub trait DependencyPort: Send + Sync {
    fn ensure_yt_dlp(&self) -> Result<String, DownloaderError>;
    fn ensure_ffmpeg(&self) -> Result<String, DownloaderError>;
    fn ensure_ffprobe(&self) -> Result<String, DownloaderError>;
}

pub trait SaveDialogPort: Send + Sync {
    fn choose_output_file(&self, mode: DownloadMode, preset: DownloadPreset) -> Option<String>;
}

pub trait DownloadPort: Send + Sync {
    fn run_download(
        &self,
        request: &DownloadRequest,
        ffmpeg_path: &str,
        on_progress: &mut dyn FnMut(DownloadProgress),
    ) -> Result<(), DownloaderError>;
}
