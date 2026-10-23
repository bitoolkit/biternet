#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod autostart;
mod components;
mod config;
mod network;
mod state;
mod theme;
mod tray;
mod update;
mod utils;
mod views;

use single_instance::SingleInstance;

use crate::app::App;

// single-instance uses a lock-file path on macOS while Linux and Windows use a
// name.
const SINGLE_INSTANCE_NAME: &str = "/tmp/io.mk1.bitgateway.lock";

fn main() -> iced::Result {
    let instance = SingleInstance::new(SINGLE_INSTANCE_NAME).unwrap();

    if !instance.is_single() {
        return Ok(());
    }

    iced::daemon(App::new, App::update, App::view)
        .title("BITGATEWAY")
        .theme(theme::application())
        .subscription(App::subscription)
        .settings(iced::Settings {
            default_text_size: 13.into(),
            ..Default::default()
        })
        .run()
}
