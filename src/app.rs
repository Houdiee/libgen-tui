use ratatui::widgets::TableState;
use reqwest::Client;
use tui_textarea::TextArea;

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
    pub downloading: bool,
    pub download_completed: bool,
}

impl App {
    pub fn new() -> Self {
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
            downloading: false,
            download_completed: false,
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
