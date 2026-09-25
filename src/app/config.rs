use std::path::PathBuf;

use config::{Config, File, FileFormat};
use dir::home_dir;
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

pub const DEFAULT_MIRRORS: [&str; 4] = ["libgen.li", "libgen.vg", "libgen.la", "libgen.bz"];

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub mirrors: Vec<String>,
    pub download_directory: String,
    pub max_results: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home = home_dir().expect("Failed to get user's home directory.");

        AppConfig {
            mirrors: DEFAULT_MIRRORS.iter().map(|m| m.to_string()).collect(),
            download_directory: format!("{}/libgen-tui", home.display()),
            max_results: 50,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let path = Self::path();

        if !path.exists() {
            let defaults = AppConfig::default();
            let encoded = toml::to_string(&defaults).expect("Failed to encode default config.");
            std::fs::write(&path, encoded).expect("Failed to write default config.");
        }

        Config::builder()
            .add_source(File::from(path).format(FileFormat::Toml))
            .build()
            .expect("Failed to build configuration.")
            .try_deserialize()
            .expect("Failed to deserialize config file.")
    }

    pub fn path() -> PathBuf {
        BaseDirectories::with_prefix("libgen-tui")
            .expect("Failed to resolve XDG directories.")
            .place_config_file("config.toml")
            .expect("Failed to place config file.")
    }
}
