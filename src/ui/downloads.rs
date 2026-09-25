use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::{App, Download, DownloadStatus};

const KEY_HINTS: &str = "[ h,j,k,l = left,down,right,left | Enter = submit | Space = immediate install | Tab = switch pane | / = search | q or Ctrl+C = quit ]";

fn status_cell(status: DownloadStatus) -> Cell<'static> {
    let (text, color) = match status {
        DownloadStatus::Pending => ("Downloading...", Color::Yellow),
        DownloadStatus::Completed => ("Download complete!", Color::Green),
        DownloadStatus::Failed => ("Download failed", Color::Red),
    };

    Cell::from(text).style(Style::default().fg(color))
}

fn row(download: &Download) -> Row<'static> {
    let error = download.error.clone().unwrap_or_default();

    Row::new(vec![
        Cell::from(download.title.clone()),
        status_cell(download.status),
        Cell::from(error).style(Style::default().fg(Color::Red)),
    ])
}

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let rows: Vec<Row> = app.downloads.snapshot().iter().rev().map(row).collect();

    let mirror = app
        .active_mirror
        .clone()
        .unwrap_or_else(|| "no mirror".to_string());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_top(Line::from("Downloads").left_aligned())
        .title_bottom(Line::from(KEY_HINTS).left_aligned())
        .title_bottom(Line::from(format!("[Connected to {}]", mirror)).right_aligned());

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ],
    )
    .block(block)
    .header(Row::new(vec![
        Cell::from("Title").style(Color::Cyan),
        Cell::from("Status").style(Color::Cyan),
        Cell::from("Error").style(Color::Cyan),
    ]));

    frame.render_widget(table, area);
}
