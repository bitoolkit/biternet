use std::fs;

use serde::{Deserialize, Serialize};

use super::config_path;

const CREDENTIALS_FILE: &str = "credentials.json";

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub remember_password: bool,
    pub auto_login: bool,
}

impl Credentials {
    pub fn can_login(&self) -> bool {
        !self.username.trim().is_empty() && !self.password.is_empty()
    }
}

pub fn load() -> Credentials {
    let path = config_path(CREDENTIALS_FILE);

    if !path.exists() {
        return Credentials::default();
    }

    let content = fs::read_to_string(path).unwrap();

    let mut credentials: Credentials = serde_json::from_str(&content).unwrap();

    if !credentials.remember_password {
        credentials.password.clear();
        credentials.auto_login = false;
    }
    if credentials.auto_login {
        credentials.remember_password = true;
    }

    credentials
}

pub fn save(credentials: &Credentials) {
    let saved_credentials = if credentials.remember_password {
        credentials.clone()
    } else {
        Credentials::default()
    };

    let path = config_path(CREDENTIALS_FILE);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let content = serde_json::to_string_pretty(&saved_credentials).unwrap();

    fs::write(path, content).unwrap();
}
