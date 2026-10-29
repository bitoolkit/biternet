use anyhow::{Context, Result};
use reqwest::header::ACCEPT;
use semver::Version;
use serde::Deserialize;

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/bitoolkit/biternet/releases/latest";
const USER_AGENT: &str = concat!("BITERNET/", env!("CARGO_PKG_VERSION"));

#[derive(Clone, Debug)]
pub struct UpdateInfo {
    pub version: String,
    pub html_url: String,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
}

pub async fn check_for_update() -> Result<Option<UpdateInfo>> {
    let release = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .context("创建更新检查客户端失败")?
        .get(LATEST_RELEASE_URL)
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .context("检查更新失败")?
        .error_for_status()
        .context("检查更新返回异常状态")?
        .json::<GithubRelease>()
        .await
        .context("解析更新信息失败")?;

    let Some(latest_version) = parse_release_version(&release.tag_name) else {
        return Ok(None);
    };

    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).context("解析当前版本失败")?;

    if latest_version <= current_version {
        return Ok(None);
    }

    Ok(Some(UpdateInfo {
        version: latest_version.to_string(),
        html_url: release.html_url,
    }))
}

fn parse_release_version(tag_name: &str) -> Option<Version> {
    let version = tag_name
        .strip_prefix("biternet-v")
        .or_else(|| tag_name.strip_prefix("biternet-"))
        .or_else(|| tag_name.strip_prefix("bitgateway-v"))
        .or_else(|| tag_name.strip_prefix("bitgateway-"))
        .or_else(|| tag_name.strip_prefix('v'))
        .unwrap_or(tag_name);

    Version::parse(version).ok()
}
