use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use config::{Config, File, FileFormat};
use ratatui::widgets::TableState;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tui_textarea::TextArea;
use xdg::BaseDirectories;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct App {
    pub client: Client,
    pub download_url: Option<String>,
    pub search_results: Vec<Book>,
    pub active_mirror: Option<String>,
    pub focus: Focus,
    pub search_bar: TextArea<'static>,
    pub query: Option<String>,
    pub should_quit: bool,
    pub searching: bool,
    pub table_state: TableState,
    pub show_popup: bool,
    pub downloads: Arc<Mutex<HashMap<String, DownloadStatus>>>,
    pub query_too_short: bool,
    pub first_query: bool,
    pub config: AppConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub mirrors: Vec<String>,
    pub download_directory: String,
    pub max_results: usize,
}

#[allow(dead_code)]
impl AppConfig {
    pub fn new() -> Self {
        let xdg_dirs = BaseDirectories::with_prefix("libgen-tui").unwrap();
        let config_path = xdg_dirs.place_config_file("config.toml").unwrap();

        if !config_path.exists() {
            let default_config = AppConfig {
                mirrors: vec!["libgen.is".to_string(), "libgen.rs".to_string()],
                download_directory: "libgen".to_string(),
                max_results: 50,
            };
            std::fs::write(&config_path, toml::to_string(&default_config).unwrap()).unwrap();
        }

        let s = Config::builder()
            .add_source(File::from(config_path).format(FileFormat::Toml))
            .build()
            .expect("Failed to build configuratoin.");

        s.try_deserialize()
            .expect("Failed to deserialize config file.")
    }

    pub fn config_path() -> PathBuf {
        let xdg_dirs = BaseDirectories::with_prefix("libgen-tui").unwrap();
        xdg_dirs.place_config_file("config.toml").unwrap()
    }
}

impl App {
    pub fn new() -> Self {
        let config = AppConfig::new();
        let download_dir = PathBuf::from(&config.download_directory);

        if !download_dir.exists() {
            fs::create_dir_all(&download_dir).expect("Failed to create directory to install files.")
        }

        App {
            client: Client::new(),
            download_url: None,
            search_results: Vec::new(),
            active_mirror: None,
            focus: Focus::SearchBar,
            search_bar: TextArea::default(),
            query: None,
            table_state: TableState::default(),
            should_quit: false,
            searching: false,
            show_popup: false,
            downloads: Arc::new(Mutex::new(HashMap::new())),
            query_too_short: false,
            first_query: true,
            config,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Focus {
    SearchBar,
    Table,
    PopupYes,
    PopupCancel,
    Nothing,
}

#[derive(Debug)]
pub enum DownloadStatus {
    Pending,
    Completed,
    Failed,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Book {
    pub id: String,
    pub author: String,
    pub title: String,
    pub publisher: String,
    pub year: String,
    pub pages: String,
    pub languages: String,
    pub size: String,
    pub extension: String,
    pub md5: String,
}
