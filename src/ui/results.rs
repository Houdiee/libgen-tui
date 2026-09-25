use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{App, Focus, ResultsState};
use crate::ui::theme;

const COLUMNS: [(&str, Color, u16); 8] = [
    ("Title", Color::Red, 20),
    ("Author", Color::Yellow, 20),
    ("Publisher", Color::Green, 20),
    ("Year", Color::Cyan, 10),
    ("Pages", Color::LightBlue, 10),
    ("Languages", Color::Blue, 10),
    ("Size", Color::LightMagenta, 10),
    ("Extension", Color::Magenta, 10),
];

fn placeholder(state: &ResultsState) -> Option<(String, Color)> {
    match state {
        ResultsState::Results => None,
        ResultsState::Searching => Some(("Searching...".to_string(), Color::Yellow)),
        ResultsState::QueryTooShort => Some((
            "Query must be at least 2 characters.".to_string(),
            Color::Red,
        )),
        ResultsState::Idle => Some((
            "Search for a book title (minimum 2 characters)".to_string(),
            Color::Green,
        )),
        ResultsState::NoResults => Some(("No results found.".to_string(), Color::Red)),
        ResultsState::Failed(reason) => Some((format!("Search failed: {}", reason), Color::Red)),
    }
}

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let border = theme::border_style(app, Focus::Table);

    if let Some((text, color)) = placeholder(&app.results_state) {
        let message = Paragraph::new(text)
            .style(Style::default().fg(color))
            .wrap(Wrap { trim: true })
            .block(theme::pane("Results", Style::new().white()));

        frame.render_widget(message, area);
        return;
    }

    let widths: Vec<Constraint> = COLUMNS
        .iter()
        .map(|(_, _, width)| Constraint::Percentage(*width))
        .collect();

    let header = Row::new(
        COLUMNS
            .iter()
            .map(|(name, color, _)| Cell::from(*name).fg(*color))
            .collect::<Vec<_>>(),
    );

    let rows = app.search_results.iter().map(|book| {
        Row::new(vec![
            book.title.clone(),
            book.author.clone(),
            book.publisher.clone(),
            book.year.clone(),
            book.pages.clone(),
            book.languages.clone(),
            book.size.clone(),
            book.extension.clone(),
        ])
    });

    let table = Table::new(rows, widths)
        .block(theme::pane("Results", border))
        .header(header)
        .row_highlight_style(border.add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(table, area, &mut app.table_state);
}
