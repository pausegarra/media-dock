use std::sync::Arc;

use pullyt::modules::downloader::application::use_cases::CheckForUpdatesUseCase;
use pullyt::modules::downloader::domain::entities::{ReleaseInfo, UpdateStatus};
use pullyt::modules::downloader::domain::errors::DownloaderError;
use pullyt::modules::downloader::domain::ports::ReleasePort;

struct StubRelease {
    result: Result<ReleaseInfo, DownloaderError>,
}

impl ReleasePort for StubRelease {
    fn fetch_latest_release(&self) -> Result<ReleaseInfo, DownloaderError> {
        match &self.result {
            Ok(info) => Ok(info.clone()),
            Err(err) => Err(err.clone()),
        }
    }
}

#[test]
fn returns_update_available_for_newer_version() {
    let port = StubRelease {
        result: Ok(ReleaseInfo {
            version: "1.1.0".to_string(),
            url: "https://example.com/release".to_string(),
        }),
    };

    let use_case = CheckForUpdatesUseCase::new(Arc::new(port), "1.0.0".to_string());
    let status = use_case.execute();

    assert!(matches!(status, UpdateStatus::UpdateAvailable(info) if info.version == "1.1.0"));
}

#[test]
fn returns_up_to_date_for_equal_version() {
    let port = StubRelease {
        result: Ok(ReleaseInfo {
            version: "v1.0.0".to_string(),
            url: "https://example.com/release".to_string(),
        }),
    };

    let use_case = CheckForUpdatesUseCase::new(Arc::new(port), "1.0.0".to_string());
    let status = use_case.execute();

    assert!(matches!(status, UpdateStatus::UpToDate));
}

#[test]
fn returns_up_to_date_when_release_fetch_fails() {
    let port = StubRelease {
        result: Err(DownloaderError::ReleaseCheckFailed("timeout".to_string())),
    };

    let use_case = CheckForUpdatesUseCase::new(Arc::new(port), "1.0.0".to_string());
    let status = use_case.execute();

    assert!(matches!(status, UpdateStatus::UpToDate));
}
