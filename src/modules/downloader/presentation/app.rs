use std::sync::Arc;

use iced::executor;
use iced::futures::SinkExt;
use iced::subscription;
use iced::theme::{self, Theme};
use iced::widget::{button, column, container, progress_bar, radio, row, text, text_input};
use iced::widget::svg::{Svg, Handle};
use iced::{window, Application, Color, Command, Element, Length, Settings, Subscription};

use crate::modules::downloader::application::use_cases::{
    BootstrapDependenciesUseCase, CheckForUpdatesUseCase, DependencyReport, DownloadMediaUseCase,
};
use crate::modules::downloader::domain::entities::{
    AudioQuality, DownloadMode, DownloadPreset, DownloadProgress, DownloadRequest, Provider,
    UpdateStatus, VideoQuality,
};
use crate::modules::downloader::infrastructure::dependencies::SystemDependencies;
use crate::modules::downloader::infrastructure::github_releases::GitHubReleaseAdapter;
use crate::modules::downloader::infrastructure::save_dialog::NativeSaveDialog;
use crate::modules::downloader::infrastructure::yt_dlp::YtDlpAdapter;

static LOGO_SVG: &[u8] = include_bytes!("assets/logo.svg");
static ICON_PNG: &[u8] = include_bytes!("assets/icon.png");

pub fn run() -> iced::Result {
    let icon = window::icon::from_file_data(ICON_PNG, None)
        .expect("failed to load app icon");

    let settings = Settings {
        window: window::Settings {
            size: iced::Size::new(800.0, 800.0),
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    };
    PullytApp::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    CheckUpdatesComplete(UpdateStatus),
    OpenReleaseLink,
    SkipUpdateAndContinue,
    UrlChanged(String),
    ModeChanged(DownloadMode),
    PresetChanged(DownloadPreset),
    VideoQualityChanged(VideoQuality),
    AudioQualityChanged(AudioQuality),
    BootstrapComplete(Result<DependencyReport, String>),
    DownloadPressed,
    DownloadProgressed(DownloadProgress),
    DownloadComplete(Result<(), String>),
    OpenGithub,
}

enum WorkerEvent {
    Progress(DownloadProgress),
    Done(Result<(), String>),
}

enum AppPhase {
    CheckingUpdates,
    UpdateGate {
        release_url: String,
        latest_version: String,
    },
    Main,
}

pub struct PullytApp {
    phase: AppPhase,
    current_version: String,
    url: String,
    mode: DownloadMode,
    preset: DownloadPreset,
    video_quality: VideoQuality,
    audio_quality: AudioQuality,
    progress: f32,
    status: String,
    dependency_info: String,
    pending_request: Option<DownloadRequest>,
    busy: bool,
}

impl Application for PullytApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let current_version = env!("CARGO_PKG_VERSION").to_string();

        let app = Self {
            phase: AppPhase::CheckingUpdates,
            current_version: current_version.clone(),
            url: String::new(),
            mode: DownloadMode::VideoWithAudio,
            preset: DownloadPreset::Compatibility,
            video_quality: VideoQuality::Best,
            audio_quality: AudioQuality::Best,
            progress: 0.0,
            status: "Checking for updates...".to_string(),
            dependency_info: String::new(),
            pending_request: None,
            busy: true,
        };

        let cmd = Command::perform(
            async move {
                let release_port = Arc::new(GitHubReleaseAdapter);
                let use_case = CheckForUpdatesUseCase::new(release_port, current_version);
                use_case.execute()
            },
            Message::CheckUpdatesComplete,
        );

        (app, cmd)
    }

    fn title(&self) -> String {
        "Pullyt - YouTube Downloader".to_string()
    }

    fn theme(&self) -> Self::Theme {
        Theme::TokyoNight
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CheckUpdatesComplete(status) => {
                if let UpdateStatus::UpdateAvailable(release) = status {
                    self.phase = AppPhase::UpdateGate {
                        release_url: release.url,
                        latest_version: release.version,
                    };
                    self.status = "A new version is available.".to_string();
                    self.busy = false;
                    return Command::none();
                }

                self.phase = AppPhase::Main;
                self.status = "Bootstrapping dependencies...".to_string();
                return Command::perform(
                    async move {
                        let dep = Arc::new(SystemDependencies);
                        let use_case = BootstrapDependenciesUseCase::new(dep);
                        use_case.execute().map_err(|e| e.to_string())
                    },
                    Message::BootstrapComplete,
                );
            }
            Message::OpenReleaseLink => {
                if let AppPhase::UpdateGate { ref release_url, .. } = self.phase {
                    let _ = open::that(release_url);
                }
                return Command::none();
            }
            Message::SkipUpdateAndContinue => {
                self.phase = AppPhase::Main;
                self.status = "Bootstrapping dependencies...".to_string();
                self.busy = true;
                return Command::perform(
                    async move {
                        let dep = Arc::new(SystemDependencies);
                        let use_case = BootstrapDependenciesUseCase::new(dep);
                        use_case.execute().map_err(|e| e.to_string())
                    },
                    Message::BootstrapComplete,
                );
            }
            Message::UrlChanged(v) => self.url = v,
            Message::ModeChanged(v) => self.mode = v,
            Message::PresetChanged(v) => self.preset = v,
            Message::VideoQualityChanged(v) => self.video_quality = v,
            Message::AudioQualityChanged(v) => self.audio_quality = v,
            Message::BootstrapComplete(res) => {
                self.busy = false;
                match res {
                    Ok(report) => {
                        self.status = "Ready".to_string();
                        self.dependency_info = format!(
                            "Dependencies\nyt-dlp: {}\nffmpeg: {}\nffprobe: {}",
                            report.yt_dlp, report.ffmpeg, report.ffprobe
                        );
                    }
                    Err(e) => self.status = format!("Dependency error: {e}"),
                }
            }
            Message::DownloadPressed => {
                self.busy = true;
                self.progress = 0.0;
                self.status = "Preparing download...".to_string();

                self.pending_request = Some(DownloadRequest {
                    provider: Provider::YouTube,
                    mode: self.mode,
                    preset: self.preset,
                    video_quality: self.video_quality,
                    audio_quality: self.audio_quality,
                    url: self.url.clone(),
                    output_path: String::new(),
                });
            }
            Message::DownloadProgressed(progress) => {
                self.progress = progress.fraction;
                self.status = progress.message;
            }
            Message::DownloadComplete(result) => {
                self.busy = false;
                self.pending_request = None;
                match result {
                    Ok(()) => {
                        self.progress = 1.0;
                        self.status = "Finished".to_string();
                    }
                    Err(e) => {
                        self.status = format!("Download failed: {e}");
                    }
                }
            }
            Message::OpenGithub => {
                let _ = open::that("https://github.com/pausegarra/pullyt");
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if !self.busy {
            return Subscription::none();
        }

        let Some(request) = self.pending_request.clone() else {
            return Subscription::none();
        };

        subscription::channel("download-progress", 100, move |mut output| async move {
            let (tx, rx) = std::sync::mpsc::channel::<WorkerEvent>();

            std::thread::spawn(move || {
                let dependencies = Arc::new(SystemDependencies);
                let save = Arc::new(NativeSaveDialog);
                let yt_dlp = Arc::new(YtDlpAdapter);
                let use_case = DownloadMediaUseCase::new(dependencies, save, yt_dlp);

                let result = use_case.execute(request, &mut |progress| {
                    let _ = tx.send(WorkerEvent::Progress(progress));
                });

                let _ = tx.send(WorkerEvent::Done(result.map_err(|e| e.to_string())));
            });

            while let Ok(event) = rx.recv() {
                match event {
                    WorkerEvent::Progress(progress) => {
                        let _ = output.send(Message::DownloadProgressed(progress)).await;
                    }
                    WorkerEvent::Done(result) => {
                        let _ = output.send(Message::DownloadComplete(result)).await;
                        break;
                    }
                }
            }

            iced::futures::future::pending::<std::convert::Infallible>().await
        })
    }

    fn view(&self) -> Element<'_, Message> {
        if let AppPhase::UpdateGate {
            ref release_url,
            ref latest_version,
        } = self.phase
        {
            return self.view_update_gate(release_url, latest_version);
        }

        let logo = Svg::new(Handle::from_memory(&*LOGO_SVG))
            .width(Length::Fixed(480.0))
            .height(Length::Fixed(240.0));

        let title = container(logo)
            .width(Length::Fill)
            .center_x()
            .padding([0, 0, 2, 0]);

        let provider = container(text("YouTube").size(18))
            .padding(10)
            .style(theme::Container::Box);

        let url_input = text_input("Paste YouTube URL", &self.url)
            .on_input(Message::UrlChanged)
            .padding(12)
            .size(16);

        let mode_row = row![
            radio(
                "Video + Audio",
                DownloadMode::VideoWithAudio,
                Some(self.mode),
                Message::ModeChanged
            ),
            radio(
                "Audio only (MP3)",
                DownloadMode::AudioOnlyMp3,
                Some(self.mode),
                Message::ModeChanged
            )
        ]
        .spacing(16);

        let preset_row = if self.mode == DownloadMode::VideoWithAudio {
            row![
                text("Preset").width(Length::Fixed(80.0)),
                radio(
                    "Compatibility (H.264/AAC)",
                    DownloadPreset::Compatibility,
                    Some(self.preset),
                    Message::PresetChanged
                ),
                radio(
                    "Max Quality",
                    DownloadPreset::MaxQuality,
                    Some(self.preset),
                    Message::PresetChanged
                ),
            ]
            .spacing(8)
        } else {
            row![]
        };

        let video_quality = row![
            text("Video").width(Length::Fixed(80.0)),
            radio("Best", VideoQuality::Best, Some(self.video_quality), Message::VideoQualityChanged),
            radio("1080p", VideoQuality::P1080, Some(self.video_quality), Message::VideoQualityChanged),
            radio("720p", VideoQuality::P720, Some(self.video_quality), Message::VideoQualityChanged),
            radio("480p", VideoQuality::P480, Some(self.video_quality), Message::VideoQualityChanged),
        ]
        .spacing(8);

        let audio_quality = row![
            text("Audio").width(Length::Fixed(80.0)),
            radio("Best", AudioQuality::Best, Some(self.audio_quality), Message::AudioQualityChanged),
            radio("320k", AudioQuality::K320, Some(self.audio_quality), Message::AudioQualityChanged),
            radio("192k", AudioQuality::K192, Some(self.audio_quality), Message::AudioQualityChanged),
            radio("128k", AudioQuality::K128, Some(self.audio_quality), Message::AudioQualityChanged),
        ]
        .spacing(8);

        let download_btn = if self.busy {
            button("Working...")
        } else {
            button("Download").on_press(Message::DownloadPressed)
        };

        let status_text = text(&self.status)
            .size(16)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..iced::Font::DEFAULT
            });

        let deps_text = text(&self.dependency_info)
            .size(12)
            .style(theme::Text::Color(Color::from_rgb(0.45, 0.55, 0.65)));

        let developer_info = row![
            text("developed by Pau Segarra").size(12),
            button("Github").on_press(Message::OpenGithub).padding(0),
        ]
        .spacing(6);

        let version_text = text(format!("v{}", env!("CARGO_PKG_VERSION")))
            .size(12)
            .style(theme::Text::Color(Color::from_rgb(0.45, 0.55, 0.65)));

        let footer = column![
            row![deps_text],
            row![version_text, row![].width(Length::Fill), developer_info],
        ]
        .align_items(iced::Alignment::Start);

        let body = column![
            title,
            provider,
            url_input,
            mode_row,
            preset_row,
            video_quality,
            audio_quality,
            status_text,
            progress_bar(0.0..=1.0, self.progress),
            download_btn,
            footer,
        ]
        .spacing(14)
        .padding([24, 24, 32, 24])
        .max_width(900);

        container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl PullytApp {
    fn view_update_gate<'a>(
        &'a self,
        _release_url: &'a str,
        latest_version: &'a str,
    ) -> Element<'a, Message> {
        let logo = Svg::new(Handle::from_memory(&*LOGO_SVG))
            .width(Length::Fixed(260.0))
            .height(Length::Fixed(130.0));

        let heading = text("Update available")
            .size(24)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..iced::Font::DEFAULT
            });

        let current = text(format!("Installed version: v{}", self.current_version)).size(14);
        let latest = text(format!("Latest version: v{latest_version}")).size(14);

        let download_btn = button("Download update")
            .on_press(Message::OpenReleaseLink)
            .padding(10);

        let skip_btn = button("Not now")
            .on_press(Message::SkipUpdateAndContinue)
            .padding(10);

        let actions = row![download_btn, skip_btn].spacing(12);

        let body = column![
            container(logo)
                .width(Length::Fill)
                .center_x()
                .padding([0, 0, 16, 0]),
            heading,
            current,
            latest,
            container(actions)
                .width(Length::Fill)
                .center_x()
                .padding([20, 0, 0, 0]),
        ]
        .spacing(12)
        .padding(32)
        .align_items(iced::Alignment::Center)
        .max_width(520);

        container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
