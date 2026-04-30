#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    YouTube,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadMode {
    VideoWithAudio,
    AudioOnlyMp3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoQuality {
    Best,
    P1080,
    P720,
    P480,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioQuality {
    Best,
    K320,
    K192,
    K128,
}

#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub provider: Provider,
    pub mode: DownloadMode,
    pub video_quality: VideoQuality,
    pub audio_quality: AudioQuality,
    pub url: String,
    pub output_path: String,
}

#[derive(Debug, Clone, Default)]
pub struct DownloadProgress {
    pub fraction: f32,
    pub message: String,
}
