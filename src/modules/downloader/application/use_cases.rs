use std::sync::Arc;

use crate::modules::downloader::domain::entities::{DownloadMode, DownloadProgress, DownloadRequest};
use crate::modules::downloader::domain::errors::DownloaderError;
use crate::modules::downloader::domain::ports::{DependencyPort, DownloadPort, SaveDialogPort};
use crate::modules::downloader::domain::value_objects::YoutubeUrl;

#[derive(Debug, Clone)]
pub struct DependencyReport {
    pub yt_dlp: String,
    pub ffmpeg: String,
    pub ffprobe: String,
}

pub struct BootstrapDependenciesUseCase {
    dependency_port: Arc<dyn DependencyPort>,
}

impl BootstrapDependenciesUseCase {
    pub fn new(dependency_port: Arc<dyn DependencyPort>) -> Self {
        Self { dependency_port }
    }

    pub fn execute(&self) -> Result<DependencyReport, DownloaderError> {
        let yt_dlp = self.dependency_port.ensure_yt_dlp()?;
        let ffmpeg = self.dependency_port.ensure_ffmpeg()?;
        let ffprobe = self.dependency_port.ensure_ffprobe()?;
        Ok(DependencyReport {
            yt_dlp,
            ffmpeg,
            ffprobe,
        })
    }
}

pub struct DownloadMediaUseCase {
    dependency_port: Arc<dyn DependencyPort>,
    save_dialog_port: Arc<dyn SaveDialogPort>,
    download_port: Arc<dyn DownloadPort>,
}

impl DownloadMediaUseCase {
    pub fn new(
        dependency_port: Arc<dyn DependencyPort>,
        save_dialog_port: Arc<dyn SaveDialogPort>,
        download_port: Arc<dyn DownloadPort>,
    ) -> Self {
        Self {
            dependency_port,
            save_dialog_port,
            download_port,
        }
    }

    pub fn execute(
        &self,
        mut request: DownloadRequest,
        on_progress: &mut dyn FnMut(DownloadProgress),
    ) -> Result<(), DownloaderError> {
        let valid = YoutubeUrl::parse(&request.url)?;
        request.url = valid.as_str().to_string();

        on_progress(DownloadProgress {
            fraction: 0.0,
            message: "Checking dependencies".to_string(),
        });
        self.dependency_port.ensure_yt_dlp()?;
        let ffmpeg_path = self.dependency_port.ensure_ffmpeg()?;
        self.dependency_port.ensure_ffprobe()?;

        let out = self
            .save_dialog_port
            .choose_output_file(request.mode == DownloadMode::AudioOnlyMp3)
            .ok_or(DownloaderError::SaveCanceled)?;
        request.output_path = out;

        self.download_port
            .run_download(&request, &ffmpeg_path, on_progress)
    }
}
