use std::future::Future;
use std::sync::LazyLock;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use biternet_client::{Client, LoginState, PortalResponse};

use crate::state::OnlineInfo;
use crate::utils::{format_balance, format_flux, humanize_duration};

const CHECK_TIMEOUT: Duration = Duration::from_secs(8);
const LOGIN_TIMEOUT: Duration = Duration::from_secs(10);
const LOGOUT_TIMEOUT: Duration = Duration::from_secs(10);
const LOGIN_CONFIRM_ATTEMPTS: usize = 8;
const LOGIN_CONFIRM_DELAY: Duration = Duration::from_millis(500);

static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

#[derive(Clone, Debug)]
pub enum LoginStatus {
    Online(OnlineInfo),
    Offline(String),
}

pub async fn check_status() -> Result<LoginStatus> {
    let state = with_timeout(CHECK_TIMEOUT, "登录状态检查超时", async {
        CLIENT.get_login_state().await.context("获取登录状态失败")
    })
    .await?;

    Ok(state.into())
}

pub async fn login(username: String, password: String) -> Result<OnlineInfo> {
    let result = with_timeout(LOGIN_TIMEOUT, "登录请求超时", async move {
        CLIENT
            .login(&username, &password)
            .await
            .context("登录请求失败")
    })
    .await
    .and_then(|response| check_response(response, "登录失败"));

    if let Err(error) = result {
        return match check_status().await {
            Ok(LoginStatus::Online(info)) => Ok(info),
            _ => Err(error),
        };
    }

    let mut status = check_status().await;

    for _ in 1..LOGIN_CONFIRM_ATTEMPTS {
        if matches!(&status, Ok(LoginStatus::Online(_))) {
            break;
        }

        tokio::time::sleep(LOGIN_CONFIRM_DELAY).await;
        status = check_status().await;
    }

    match status? {
        LoginStatus::Online(info) => Ok(info),
        LoginStatus::Offline(message) => bail!(message),
    }
}

pub async fn logout(username: String) -> Result<()> {
    let result = with_timeout(LOGOUT_TIMEOUT, "注销请求超时", async move {
        CLIENT.logout(&username).await.context("注销请求失败")
    })
    .await
    .and_then(|response| check_response(response, "注销失败"));

    if let Err(error) = result {
        return match check_status().await {
            Ok(LoginStatus::Offline(_)) => Ok(()),
            _ => Err(error),
        };
    }

    Ok(())
}

fn check_response(response: PortalResponse, fallback: &str) -> Result<()> {
    if response.error == "ok" || response.suc_msg.is_some() {
        return Ok(());
    }

    let message = if !response.error_msg.trim().is_empty() {
        response.error_msg
    } else if !response.error.trim().is_empty() {
        response.error
    } else {
        fallback.to_string()
    };

    bail!(message)
}

async fn with_timeout<T>(
    duration: Duration,
    message: &'static str,
    future: impl Future<Output = Result<T>>,
) -> Result<T> {
    tokio::time::timeout(duration, future)
        .await
        .context(message)?
}

impl From<LoginState> for LoginStatus {
    fn from(state: LoginState) -> Self {
        let online =
            state.error == "ok" || state.user_name.is_some() || state.sum_seconds.is_some();

        if online {
            return Self::Online(OnlineInfo {
                username: state.user_name.unwrap_or("未知用户".to_string()),
                ip: state.online_ip.to_string(),
                used_flux: format_flux(state.sum_bytes.unwrap_or(0)),
                used_duration: humanize_duration(state.sum_seconds.unwrap_or(0)),
                balance: state
                    .user_balance
                    .map(format_balance)
                    .unwrap_or("-".to_string()),
            });
        }

        let message = state
            .error_msg
            .filter(|message| !message.trim().is_empty())
            .unwrap_or("当前未登录".to_string());

        Self::Offline(message)
    }
}
