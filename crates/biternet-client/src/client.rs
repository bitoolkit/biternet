use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use bon::Builder;
use reqwest::Client as HttpClient;
use tokio::sync::OnceCell;

use crate::config::Config;
use crate::crypto::{SRUN_N, SRUN_TYPE, checksum, dm_logout_sign, hmd5, login_info};
use crate::models::{Challenge, LoginState, PortalResponse};

const JSONP_CALLBACK: &str = "jsonp";
const LOGIN_ACTION: &str = "login";
const LOGOUT_ACTION: &str = "logout";

#[derive(Debug, Builder)]
pub struct Client {
    #[builder(default = HttpClient::new())]
    http_client: HttpClient,
    #[builder(default)]
    config: Config,
    #[builder(skip = OnceCell::const_new())]
    ac_id: OnceCell<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub async fn get_login_state(&self) -> Result<LoginState> {
        let params = [("callback", JSONP_CALLBACK)];
        let raw_text = self
            .http_client
            .get(self.endpoint("/cgi-bin/rad_user_info"))
            .query(&params)
            .send()
            .await
            .context("failed to get login state")?
            .text()
            .await
            .context("failed to read login state response")?;

        let raw_json = jsonp_json(&raw_text, "login status")?;

        serde_json::from_str::<LoginState>(raw_json)
            .with_context(|| format!("failed to parse login status response: {raw_json}"))
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<PortalResponse> {
        self._login(username, password, false).await
    }

    pub async fn force_login(&self, username: &str, password: &str) -> Result<PortalResponse> {
        self._login(username, password, true).await
    }

    pub async fn logout(&self, username: &str) -> Result<PortalResponse> {
        self._logout(username, false).await
    }

    pub async fn force_logout(&self, username: &str) -> Result<PortalResponse> {
        self._logout(username, true).await
    }

    async fn _login(&self, username: &str, password: &str, force: bool) -> Result<PortalResponse> {
        let login_state = self.get_login_state().await?;
        if login_state.error == "ok" && !force {
            bail!("{} already logged in", login_state.online_ip);
        }

        let ip = self.request_ip(&login_state);
        let ip_string = ip.to_string();
        let ac_id = self.ac_id().await?.to_string();
        let token = self.challenge(username, ip).await?;
        let info = login_info(username, password, &ip_string, &ac_id, &token)?;
        let hmd5 = hmd5(&token)?;
        let chksum = checksum(&token, username, &hmd5, &ac_id, &ip_string, &info);
        let encoded_password = format!("{}{}", "{MD5}", hmd5);
        let params = [
            ("callback", JSONP_CALLBACK),
            ("action", LOGIN_ACTION),
            ("username", username),
            ("password", encoded_password.as_str()),
            ("chksum", chksum.as_str()),
            ("info", info.as_str()),
            ("ac_id", ac_id.as_str()),
            ("ip", ip_string.as_str()),
            ("type", SRUN_TYPE),
            ("n", SRUN_N),
        ];
        let raw_text = self
            .http_client
            .get(self.endpoint("/cgi-bin/srun_portal"))
            .query(&params)
            .send()
            .await
            .context("failed to send login request")?
            .text()
            .await
            .context("failed to read login response")?;

        let raw_json = jsonp_json(&raw_text, "login")?;

        serde_json::from_str::<PortalResponse>(raw_json)
            .with_context(|| format!("failed to parse login response: {raw_json}"))
    }

    async fn _logout(&self, username: &str, force: bool) -> Result<PortalResponse> {
        let login_state = self.get_login_state().await?;
        if login_state.error == "not_online_error" && !force {
            bail!("{} already logged out", login_state.online_ip);
        }

        let logged_in_username = login_state
            .user_name
            .clone()
            .unwrap_or_else(|| username.to_string());
        let ip = self.request_ip(&login_state);
        let ip_string = ip.to_string();
        let mut params = vec![
            ("callback", JSONP_CALLBACK.to_string()),
            ("ip", ip_string.clone()),
            ("username", logged_in_username.clone()),
        ];
        let endpoint = if self.config.dumb_terminal() {
            let timestamp = current_timestamp();
            let unbind = "1".to_string();
            let sign = dm_logout_sign(&timestamp, &logged_in_username, &ip_string, &unbind);

            params.push(("time", timestamp));
            params.push(("unbind", unbind));
            params.push(("sign", sign));

            "/cgi-bin/rad_user_dm"
        } else {
            params.push(("action", LOGOUT_ACTION.to_string()));
            params.push(("ac_id", self.ac_id().await?.to_string()));

            "/cgi-bin/srun_portal"
        };
        let raw_text = self
            .http_client
            .get(self.endpoint(endpoint))
            .query(&params)
            .send()
            .await
            .context("failed to send logout request")?
            .text()
            .await
            .context("failed to read logout response")?;

        let raw_json = jsonp_json(&raw_text, "logout")?;

        serde_json::from_str::<PortalResponse>(raw_json)
            .with_context(|| format!("failed to parse logout response: {raw_json}"))
    }

    async fn ac_id(&self) -> Result<&str> {
        let ac_id = self
            .ac_id
            .get_or_try_init(|| async { self.discover_ac_id().await })
            .await?;

        Ok(ac_id)
    }

    async fn discover_ac_id(&self) -> Result<String> {
        match self.ac_id_by_url(self.config.captive_portal_url()).await {
            Ok(ac_id) => Ok(ac_id),
            Err(_) => self.ac_id_by_url(self.config.portal_url()).await,
        }
    }

    async fn ac_id_by_url(&self, url: &str) -> Result<String> {
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .with_context(|| format!("failed to get ac_id from `{url}`"))?;

        let redirect_url = response.url();
        let Some((_, ac_id)) = redirect_url.query_pairs().find(|(key, _)| key == "ac_id") else {
            bail!("failed to get ac_id from `{redirect_url}`");
        };

        Ok(ac_id.into_owned())
    }

    async fn challenge(&self, username: &str, ip: IpAddr) -> Result<String> {
        let ip_string = ip.to_string();
        let params = [
            ("callback", JSONP_CALLBACK),
            ("username", username),
            ("ip", ip_string.as_str()),
        ];
        let raw_text = self
            .http_client
            .get(self.endpoint("/cgi-bin/get_challenge"))
            .query(&params)
            .send()
            .await
            .context("failed to get challenge")?
            .text()
            .await
            .context("failed to read challenge response")?;

        let raw_json = jsonp_json(&raw_text, "challenge")?;
        let challenge = serde_json::from_str::<Challenge>(raw_json)
            .with_context(|| format!("failed to parse challenge response: {raw_json}"))?;

        Ok(challenge.challenge)
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.config.portal_url().trim_end_matches('/'), path)
    }

    fn request_ip(&self, login_state: &LoginState) -> IpAddr {
        self.config.ip().unwrap_or(login_state.online_ip)
    }
}

fn jsonp_json<'a>(raw_text: &'a str, label: &str) -> Result<&'a str> {
    let Some(raw_json) = raw_text
        .strip_prefix("jsonp(")
        .and_then(|text| text.strip_suffix(')'))
    else {
        bail!("{label} response is not valid jsonp: `{raw_text}`");
    };

    Ok(raw_json)
}

fn current_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}
