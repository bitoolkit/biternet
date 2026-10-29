use anyhow::Result;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha1::{Digest, Sha1};

use crate::xencode::{fkbase64, xencode};

pub const SRUN_TYPE: &str = "1";
pub const SRUN_N: &str = "200";

pub fn login_info(
    username: &str,
    password: &str,
    ip: &str,
    ac_id: &str,
    token: &str,
) -> Result<String> {
    let checksum_data = serde_json::json!({
        "username": username,
        "password": password,
        "ip": ip,
        "acid": ac_id,
        "enc_ver": "srun_bx1",
    });
    let json_checksum_data = serde_json::to_string(&checksum_data)?;

    Ok(format!(
        "{}{}",
        "{SRBX1}",
        fkbase64(xencode(&json_checksum_data, token))
    ))
}

pub fn hmd5(token: &str) -> Result<String> {
    let mac = Hmac::<Md5>::new_from_slice(token.as_bytes())?;

    Ok(format!("{:x}", mac.finalize().into_bytes()))
}

pub fn checksum(
    token: &str,
    username: &str,
    hmd5: &str,
    ac_id: &str,
    ip: &str,
    info: &str,
) -> String {
    let source = format!(
        "{token}{username}{token}{hmd5}{token}{ac_id}{token}{ip}{token}{SRUN_N}{token}{SRUN_TYPE}{token}{info}"
    );
    let mut hasher = Sha1::new();
    hasher.update(source);

    format!("{:x}", hasher.finalize())
}

pub fn dm_logout_sign(timestamp: &str, username: &str, ip: &str, unbind: &str) -> String {
    let source = format!("{timestamp}{username}{ip}{unbind}{timestamp}");
    let mut hasher = Sha1::new();
    hasher.update(source);

    format!("{:x}", hasher.finalize())
}
