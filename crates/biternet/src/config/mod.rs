use std::path::PathBuf;

use directories::ProjectDirs;

pub mod credentials;
pub mod settings;

pub use credentials::Credentials;
pub use settings::Settings;

fn config_path(file_name: &str) -> PathBuf {
    // Keep existing credentials and settings available after the rename.
    let project_dirs = ProjectDirs::from("dev", "so1ve", "bitgateway").unwrap();

    project_dirs.config_dir().join(file_name)
}
