use std::env;

use active_mirror::check_mirrors_and_return_active;
use app::{App, Focus};
use download::download_book;
use download_url::return_download_url;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    widgets::{block::Title, Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    DefaultTerminal, Frame,
};
use search::return_books_from_search;

mod active_mirror;
mod app;
mod download;
mod download_url;
mod search;

#[tokio::main]
async fn main() {
    env::set_var("LOG_LEVEL", "TRACE");
    env_logger::init();

    let mut app = App::new();

    let mirrors = vec!["libgen.lol", "libgen.is", "libgen.rs"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let mirror = tokio::spawn(check_mirrors_and_return_active(app.client.clone(), mirrors))
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
                            app.focus = Focus::Table;
                            app.search_bar
                                .set_cursor_style(Style::default().bg(Color::Reset));
                        }
                        KeyCode::Enter => {
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

                            let results = return_books_from_search(&mirror, &query, client)
                                .await
                                .unwrap();

                            if !results.is_empty() {
                                app.table_state.select(Some(0));
                            } else {
                                app.table_state.select(None);
                                app.focus = Focus::SearchBar;
                            }

                            app.search_results = results;
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
                    KeyCode::Tab => {
                        app.focus = Focus::PopupCancel;
                    }

                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.show_popup = false;
                        app.focus = Focus::Table;
                    }

                    KeyCode::Enter => {
                        if let Some(selected) = app.table_state.selected() {
                            let selected_book = app.search_results[selected].clone();
                            let md5 = selected_book.md5;

                            let client_clone = app.client.clone();
                            tokio::spawn(async move {
                                match return_download_url(md5.clone(), client_clone).await {
                                    Ok(url) => {
                                        let filename =
                                            format!("{}.{}", md5, selected_book.extension);
                                        let destination = filename.as_str();

                                        if let Err(e) = download_book(&url, destination).await {
                                            eprintln!("Error downloading book: {}", e);
                                        } else {
                                            println!("Download complete: {}", destination);
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error getting download URL: {}", e);
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
                    KeyCode::Tab | KeyCode::Char('q') => {
                        app.focus = Focus::PopupYes;
                    }
                    KeyCode::Esc => {
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

pub fn draw(frame: &mut Frame, app: &mut App) {
    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Percentage(90),
        Constraint::Percentage(10),
    ]);
    let chunks = layout.split(frame.area());

    let mut search_bar = app.search_bar.clone();
    let search_bar_border_style = return_border_color(app, Focus::SearchBar);
    search_bar.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(search_bar_border_style)
            .title(Title::from("Search"))
            .title_alignment(Alignment::Left),
    );
    search_bar.set_placeholder_text("Title");
    frame.render_widget(&search_bar, chunks[0]);

    let rows: Vec<_> = app
        .search_results
        .iter()
        .map(|b| {
            Row::new(vec![
                b.clone().title,
                b.clone().author,
                b.clone().publisher,
                b.clone().year,
                b.clone().pages,
                b.clone().languages,
                b.clone().size,
                b.clone().extension,
            ])
        })
        .collect();

    let header = [
        Cell::from("Title").fg(Color::Red),
        Cell::from("Author").fg(Color::Yellow),
        Cell::from("Publisher").fg(Color::Green),
        Cell::from("Year").fg(Color::Cyan),
        Cell::from("Pages").fg(Color::LightBlue),
        Cell::from("Languages").fg(Color::Blue),
        Cell::from("Size").fg(Color::LightMagenta),
        Cell::from("Extension").fg(Color::Magenta),
    ];

    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
    ];

    let table_border_style = return_border_color(&app, Focus::Table);
    let table = Table::new(rows, widths)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(table_border_style)
                .title(Title::from("Results"))
                .title_alignment(Alignment::Left)
                .title_bottom(
                    format!("[Connected to {}]", app.active_mirror.clone().unwrap()).white(),
                ),
        )
        .widths(widths)
        .row_highlight_style(table_border_style.add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
        .header(Row::new(header));

    let loading = Paragraph::new("Searching...")
        .set_style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().white())
                .title(Title::from("Results"))
                .title_alignment(Alignment::Left)
                .title_bottom(
                    format!("[Connected to {}]", app.active_mirror.clone().unwrap()).white(),
                ),
        );

    if app.show_popup {
        if let Some(index) = app.table_state.selected() {
            let selected_book = app.search_results[index].clone();

            let popup_msg = format!(
                "Confirm installation for '{}' by '{}'?",
                selected_book.title, selected_book.author
            );

            let block = Paragraph::new(popup_msg)
                .centered()
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().blue()),
                );

            let area = popup_area(frame.area(), 30, 25);
            frame.render_widget(Clear, area);

            let inner_layout = Layout::new(
                Direction::Vertical,
                [Constraint::Percentage(75), Constraint::Percentage(25)],
            )
            .split(area);

            frame.render_widget(block, inner_layout[0]);

            let button_layout = Layout::new(
                Direction::Horizontal,
                [Constraint::Percentage(50), Constraint::Percentage(50)],
            )
            .split(inner_layout[1]);

            let cancel_button_style = return_border_color(app, Focus::PopupCancel);
            let cancel_button = Paragraph::new("Cancel")
                .centered()
                .set_style(Color::Red)
                .block(
                    Block::default()
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded)
                        .border_style(cancel_button_style),
                );

            let yes_button_style = return_border_color(app, Focus::PopupYes);
            let yes_button = Paragraph::new("Install")
                .centered()
                .set_style(Color::Green)
                .block(
                    Block::default()
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded)
                        .border_style(yes_button_style),
                );

            frame.render_widget(cancel_button, button_layout[0]);
            frame.render_widget(yes_button, button_layout[1]);
        }
    }

    if app.searching {
        frame.render_widget(loading, chunks[1]);
    } else {
        frame.render_stateful_widget(table, chunks[1], &mut app.table_state)
    }
}

pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn return_border_color(app: &App, focus_target: Focus) -> Style {
    let mut focused_color = Style::new().blue();
    let unfocused_color = Style::new().white();

    if focus_target == Focus::PopupYes {
        focused_color = Style::new().green();
    }
    if focus_target == Focus::PopupCancel {
        focused_color = Style::new().green();
    }

    if app.focus == focus_target {
        focused_color
    } else {
        unfocused_color
    }
}

