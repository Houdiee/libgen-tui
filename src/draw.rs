use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{App, Focus};
use crate::DownloadStatus;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Percentage(70),
        Constraint::Percentage(30),
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
                .title_alignment(Alignment::Left),
        )
        .widths(widths)
        .row_highlight_style(table_border_style.add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
        .header(Row::new(header));

    let (text, style) = if app.searching {
        ("Searching...", Color::Yellow)
    } else if app.query_too_short {
        ("Query must be at least 2 characters.", Color::Red)
    } else if app.first_query {
        (
            "Search for a book title (minimum 2 characters)",
            Color::Green,
        )
    } else {
        ("No results found.", Color::Red)
    };

    let loading = Paragraph::new(text)
        .set_style(Style::default().fg(style))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().white())
                .title(Title::from("Results"))
                .title_alignment(Alignment::Left),
        );

    if app.searching || app.search_results.is_empty() {
        frame.render_widget(loading, chunks[1]);
    } else {
        frame.render_stateful_widget(table, chunks[1], &mut app.table_state)
    }

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

    let downloads_lock = app.downloads.lock().unwrap();

    let mut downloads_rows: Vec<_> = downloads_lock
        .iter()
        .map(|((title, _md5), completed)| {
            let (text, style) = match completed {
                DownloadStatus::Pending => ("Downloading...", Color::Yellow),
                DownloadStatus::Completed => ("Download complete!", Color::Green),
                DownloadStatus::Failed => ("Download failed", Color::Red),
            };

            Row::new(vec![
                Cell::from(title.clone()),
                Cell::from(text).style(style),
            ])
        })
        .collect();
    downloads_rows.reverse();

    let downloads_table = Table::new(
        downloads_rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_top(Line::from("Downloads").left_aligned())
            .title_bottom(
                Line::from(
                    "[ h,j,k,l = left,down,right,left | Enter = submit | Space = immediate install | Tab = switch pane | / = search | q = quit ]",
                )
                .left_aligned(),
            )
            .title_bottom(
                Line::from(format!(
                    "[Connected to {}]",
                    app.active_mirror.clone().unwrap()
                ))
                .right_aligned(),
            ),
    )
    .header(Row::new(vec![
        Cell::from("Title").style(Color::Cyan),
        Cell::from("Status").style(Color::Cyan),
    ]));

    frame.render_widget(downloads_table, chunks[2]);
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
        focused_color = Style::new().red();
    }

    if app.focus == focus_target {
        focused_color
    } else {
        unfocused_color
    }
}
