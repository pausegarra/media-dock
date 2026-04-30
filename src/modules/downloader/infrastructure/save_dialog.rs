use rfd::FileDialog;

use crate::modules::downloader::domain::ports::SaveDialogPort;

pub struct NativeSaveDialog;

impl SaveDialogPort for NativeSaveDialog {
    fn choose_output_file(&self, audio_only: bool) -> Option<String> {
        let mut dialog = FileDialog::new();
        dialog = if audio_only {
            dialog.add_filter("MP3 audio", &["mp3"]).set_file_name("media-dock.mp3")
        } else {
            dialog
                .add_filter("Video", &["mp4", "mkv"])
                .set_file_name("media-dock.mp4")
        };
        dialog.save_file().map(|p| p.display().to_string())
    }
}
