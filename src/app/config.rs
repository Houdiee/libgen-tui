use std::path::PathBuf;

use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

pub const DEFAULT_MIRRORS: [&str; 5] = [
    "libgen.li",
    "libgen.la",
    "libgen.bz",
    "libgen.gl",
    "libgen.vg",
];

pub fn default_mirrors() -> Vec<String> {
    DEFAULT_MIRRORS.iter().map(|m| m.to_string()).collect()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub additional_mirrors: Vec<String>,
    pub download_directory: String,
    pub max_results: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home = dirs::home_dir().expect("Failed to get user's home directory.");

        AppConfig {
            additional_mirrors: Vec::new(),
            download_directory: home.join("libgen-tui").display().to_string(),
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

    pub fn directory() -> PathBuf {
        dirs::config_dir()
            .expect("Failed to resolve the configuration directory.")
            .join("libgen-tui")
    }

    pub fn path() -> PathBuf {
        let directory = Self::directory();
        std::fs::create_dir_all(&directory).expect("Failed to create the configuration directory.");
        directory.join("config.toml")
    }
}
