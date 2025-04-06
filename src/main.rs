use std::sync::Arc;

use futures::lock::Mutex;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::{Color, Style},
    DefaultTerminal,
};

use std::env;

const MIN_QUERY_LEN: usize = 2;

use active_mirror::check_mirrors_and_return_active;
use app::{App, DownloadStatus, Focus};
use download::download_book;
use download_url::return_download_url;
use draw::draw;
use search::return_books_from_search;

mod active_mirror;
mod app;
mod download;
mod download_url;
mod draw;
mod search;

#[tokio::main]
async fn main() {
    env::set_var("LOG_LEVEL", "TRACE");
    env_logger::init();

    let mut app = App::new();

    let mirror = tokio::spawn(check_mirrors_and_return_active(
        app.client.clone(),
        app.config.mirrors.clone(),
    ))
    .await
    .expect("Failed to check_mirrors.");

    match &mirror {
        Ok(m) => app.active_mirror = Some(String::from(m)),
        Err(_) => app.active_mirror = None,
    }

    let terminal = ratatui::init();
    run(terminal, &mut app).await;
    ratatui::restore();
}

pub async fn run(mut terminal: DefaultTerminal, app: &mut App) {
    loop {
        terminal
            .draw(|frame| draw(frame, app))
            .expect("Failed to draw to terminal.");

        if let Event::Key(key) = event::read().expect("Failed to read event.") {
            match app.focus {
                Focus::SearchBar => {
                    app.search_bar
                        .set_cursor_style(Style::default().bg(Color::White));

                    match key.code {
                        KeyCode::Esc => app.focus = Focus::Nothing,
                        KeyCode::Tab => {
                            if !app.search_results.is_empty() {
                                app.search_bar
                                    .set_cursor_style(Style::default().bg(Color::Reset));
                                app.focus = Focus::Table;
                            }
                        }
                        KeyCode::Enter => {
                            app.first_query = false;
                            app.query_too_short = false;
                            app.searching = true;
                            app.focus = Focus::Table;
                            app.search_bar
                                .set_cursor_style(Style::default().bg(Color::Reset));

                            let _ = terminal.draw(|frame| {
                                let mut app = app.clone();
                                draw(frame, &mut app);
                            });

                            let query = app.search_bar.lines().to_owned();
                            assert_eq!(query.len(), 1);
                            app.query = Some(query[0].clone());

                            let mirror = app.active_mirror.to_owned().unwrap();
                            let query = app.query.to_owned().unwrap();
                            let client = app.client.to_owned();

                            if query.len() >= MIN_QUERY_LEN {
                                let results = return_books_from_search(
                                    &mirror,
                                    &query,
                                    client,
                                    app.config.max_results.clone(),
                                )
                                .await
                                .unwrap();

                                if !results.is_empty() {
                                    app.table_state.select(Some(0));
                                    app.focus = Focus::Table;
                                } else {
                                    app.table_state.select(None);
                                    app.focus = Focus::SearchBar;
                                }

                                app.search_results = results;
                            } else {
                                app.query_too_short = true;
                                app.focus = Focus::SearchBar;
                            }
                            app.searching = false;
                        }
                        _ => {
                            app.search_bar.input(key);
                        }
                    }
                }

                Focus::Nothing => match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Tab => app.focus = Focus::SearchBar,
                    KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Down | KeyCode::Up => {
                        app.focus = Focus::Table
                    }
                    _ => {}
                },

                Focus::Table => match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Tab => app.focus = Focus::SearchBar,
                    KeyCode::Esc => app.focus = Focus::Nothing,
                    KeyCode::Char('j') | KeyCode::Down => {
                        if let Some(index) = app.table_state.selected() {
                            if app.search_results.len() - 1 > index {
                                let increment_index = index + 1;
                                app.table_state.select(Some(increment_index));
                            }
                        }
                    }

                    KeyCode::Char('k') | KeyCode::Up => {
                        if let Some(index) = app.table_state.selected() {
                            if index > 0 {
                                let decrement_index = index - 1;
                                app.table_state.select(Some(decrement_index));
                            }
                        }
                    }

                    KeyCode::Enter => {
                        if let Some(_) = app.table_state.selected() {
                            app.show_popup = true;
                            app.focus = Focus::PopupYes;
                        }
                    }
                    _ => {}
                },

                Focus::PopupYes => match key.code {
                    KeyCode::Tab | KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('j') => {
                        app.focus = Focus::PopupCancel;
                    }

                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.show_popup = false;
                        app.focus = Focus::Table;
                    }

                    KeyCode::Enter => {
                        if let Some(selected) = app.table_state.selected() {
                            let selected_book = app.search_results[selected].clone();
                            let title = selected_book.title.clone();
                            let md5 = selected_book.md5.clone();
                            let client_clone = app.client.clone();
                            let extension = selected_book.extension.clone();
                            let download_dir = app.config.download_directory.clone();

                            let downloads = Arc::clone(&app.downloads);
                            app.downloads
                                .lock()
                                .unwrap()
                                .insert(title.clone(), DownloadStatus::Pending);

                            tokio::spawn(async move {
                                match return_download_url(md5.clone(), client_clone).await {
                                    Ok(url) => {
                                        let filename = format!("{}.{}", md5, extension);

                                        let destination = if download_dir.ends_with("/") {
                                            format!("{}{}", download_dir, filename)
                                        } else {
                                            format!("{}/{}", download_dir, filename)
                                        };

                                        if let Err(e) = download_book(&url, &destination).await {
                                            eprintln!("Error downloading book: {}", e);
                                        } else {
                                            downloads
                                                .lock()
                                                .unwrap()
                                                .insert(title.clone(), DownloadStatus::Completed);
                                        }
                                    }
                                    Err(_) => {
                                        downloads
                                            .lock()
                                            .unwrap()
                                            .insert(title.clone(), DownloadStatus::Failed);
                                    }
                                }
                            });
                        }
                        app.show_popup = false;
                        app.focus = Focus::Table;
                    }
                    _ => {}
                },

                Focus::PopupCancel => match key.code {
                    KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('k') => {
                        app.focus = Focus::PopupYes;
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.show_popup = false;
                        app.focus = Focus::Table;
                    }
                    KeyCode::Enter => {
                        app.show_popup = false;
                        app.focus = Focus::Table;
                    }
                    _ => {}
                },
            }
        }

        if app.should_quit {
            break;
        }
    }
}
