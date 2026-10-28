use std::fs;

use serde::{Deserialize, Serialize};

use super::config_path;

const SETTINGS_FILE: &str = "settings.json";

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    pub auto_start: bool,
    pub silent_start: bool,
}

pub fn load() -> Settings {
    let path = config_path(SETTINGS_FILE);

    if !path.exists() {
        return Settings::default();
    }

    let content = fs::read_to_string(path).unwrap();

    serde_json::from_str(&content).unwrap()
}

pub fn save(settings: &Settings) {
    let path = config_path(SETTINGS_FILE);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let content = serde_json::to_string_pretty(settings).unwrap();

    fs::write(path, content).unwrap();
}
