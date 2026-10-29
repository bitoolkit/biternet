use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginState {
    pub error: String,
    pub online_ip: IpAddr,

    #[serde(rename = "ServerFlag", skip_serializing_if = "Option::is_none")]
    pub server_flag: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepalive_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_charge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_balance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srun_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortalResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suc_msg: Option<String>,

    pub client_ip: IpAddr,
    pub online_ip: IpAddr,
    pub error: String,
    pub error_msg: String,
    pub res: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Challenge {
    pub challenge: String,
}
