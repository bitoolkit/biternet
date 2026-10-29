use anyhow::{Context, Result};
use auto_launch::{
    AutoLaunch, AutoLaunchBuilder, LinuxLaunchMode, MacOSLaunchMode, WindowsEnableMode,
};

use crate::config::Settings;

// Keep the registration name so existing startup entries can be updated or
// removed.
const AUTOSTART_NAME: &str = "BITGATEWAY";

fn build_auto_launch(silent_start: bool) -> AutoLaunch {
    let app_path = std::env::current_exe()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let args = if silent_start {
        vec!["--silent".to_string()]
    } else {
        Vec::new()
    };

    AutoLaunchBuilder::new()
        .set_app_name(AUTOSTART_NAME)
        .set_app_path(&app_path)
        .set_args(&args)
        .set_linux_launch_mode(LinuxLaunchMode::XdgAutostart)
        .set_macos_launch_mode(MacOSLaunchMode::LaunchAgent)
        .set_windows_enable_mode(WindowsEnableMode::CurrentUser)
        .build()
        .unwrap()
}

pub fn sync(settings: &Settings) -> Result<()> {
    let launcher = build_auto_launch(settings.silent_start);

    if settings.auto_start {
        launcher.enable().context("注册开机自动启动失败")?;
    } else {
        launcher.disable().context("取消开机自动启动失败")?;
    }

    Ok(())
}
