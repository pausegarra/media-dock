use std::sync::Arc;

use pullryn::modules::downloader::application::use_cases::DownloadMediaUseCase;
use pullryn::modules::downloader::domain::entities::{
    AudioQuality, DownloadMode, DownloadPreset, DownloadProgress, DownloadRequest, Provider,
    VideoQuality,
};
use pullryn::modules::downloader::domain::errors::DownloaderError;
use pullryn::modules::downloader::domain::ports::{DependencyPort, DownloadPort, SaveDialogPort};

struct OkDeps;
impl DependencyPort for OkDeps {
    fn ensure_yt_dlp(&self) -> Result<String, DownloaderError> {
        Ok("yt-dlp".to_string())
    }
    fn ensure_ffmpeg(&self) -> Result<String, DownloaderError> {
        Ok("ffmpeg".to_string())
    }
    fn ensure_ffprobe(&self) -> Result<String, DownloaderError> {
        Ok("ffprobe".to_string())
    }
}

struct FakeDialog;
impl SaveDialogPort for FakeDialog {
    fn choose_output_file(
        &self,
        _mode: DownloadMode,
        _preset: DownloadPreset,
        _title: &str,
    ) -> Option<String> {
        Some("/tmp/out.mp4".to_string())
    }
}

struct FakeDownload;
impl DownloadPort for FakeDownload {
    fn run_download(
        &self,
        _request: &DownloadRequest,
        _ffmpeg_path: &str,
        on_progress: &mut dyn FnMut(DownloadProgress),
    ) -> Result<(), DownloaderError> {
        on_progress(DownloadProgress {
            fraction: 1.0,
            message: "Finished".to_string(),
        });
        Ok(())
    }

    fn get_title(&self, _url: &str) -> Result<String, DownloaderError> {
        Ok("Test Video Title".to_string())
    }
}

#[test]
fn rejects_invalid_url() {
    let use_case = DownloadMediaUseCase::new(Arc::new(OkDeps), Arc::new(FakeDialog), Arc::new(FakeDownload));
    let request = DownloadRequest {
        provider: Provider::YouTube,
        mode: DownloadMode::VideoWithAudio,
        preset: DownloadPreset::Compatibility,
        video_quality: VideoQuality::Best,
        audio_quality: AudioQuality::Best,
        url: "https://example.com/not-youtube".to_string(),
        output_path: String::new(),
    };
    let result = use_case.execute(request, &mut |_| {});
    assert!(matches!(result, Err(DownloaderError::InvalidUrl)));
}
