use rfd::FileDialog;

use crate::modules::downloader::domain::entities::{DownloadMode, DownloadPreset};
use crate::modules::downloader::domain::ports::SaveDialogPort;

pub struct NativeSaveDialog;

impl SaveDialogPort for NativeSaveDialog {
    fn choose_output_file(&self, mode: DownloadMode, preset: DownloadPreset) -> Option<String> {
        let mut dialog = FileDialog::new();

        dialog = match mode {
            DownloadMode::AudioOnlyMp3 => {
                dialog
                    .add_filter("MP3 audio", &["mp3"])
                    .set_file_name("media-dock.mp3")
            }
            DownloadMode::VideoWithAudio => match preset {
                DownloadPreset::Compatibility => dialog
                    .add_filter("Video", &["mp4"])
                    .set_file_name("media-dock.mp4"),
                DownloadPreset::MaxQuality => dialog
                    .add_filter("Video", &["mkv", "webm", "mp4"])
                    .set_file_name("media-dock.mkv"),
            },
        };

        dialog.save_file().map(|p| p.display().to_string())
    }
}
