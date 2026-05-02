use serde::Deserialize;

use crate::modules::downloader::domain::entities::ReleaseInfo;
use crate::modules::downloader::domain::errors::DownloaderError;
use crate::modules::downloader::domain::ports::ReleasePort;

const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/pausegarra/pullryn/releases/latest";
const USER_AGENT: &str = "pullryn update-check";

pub struct GitHubReleaseAdapter;

impl ReleasePort for GitHubReleaseAdapter {
    fn fetch_latest_release(&self) -> Result<ReleaseInfo, DownloaderError> {
        let agent = ureq::AgentBuilder::new()
            .user_agent(USER_AGENT)
            .build();

        let response: LatestReleaseResponse = agent
            .get(LATEST_RELEASE_URL)
            .set("Accept", "application/vnd.github+json")
            .call()
            .map_err(|e| DownloaderError::ReleaseCheckFailed(e.to_string()))?
            .into_json()
            .map_err(|e| DownloaderError::ReleaseCheckFailed(e.to_string()))?;

        let version = response
            .tag_name
            .strip_prefix('v')
            .unwrap_or(&response.tag_name)
            .to_string();

        Ok(ReleaseInfo {
            version,
            url: response.html_url,
        })
    }
}

#[derive(Deserialize)]
struct LatestReleaseResponse {
    tag_name: String,
    html_url: String,
}
