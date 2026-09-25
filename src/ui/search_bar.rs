use ratatui::{
    layout::Rect,
    style::{Color, Style},
    Frame,
};

use crate::app::{App, Focus};
use crate::ui::theme;

fn cursor_style(app: &App) -> Style {
    if app.focus == Focus::SearchBar {
        Style::default().bg(Color::White)
    } else {
        Style::default().bg(Color::Reset)
    }
}

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let mut search_bar = app.search_bar.clone();

    search_bar.set_block(theme::pane(
        "Search",
        theme::border_style(app, Focus::SearchBar),
    ));
    search_bar.set_placeholder_text("Title");
    search_bar.set_cursor_style(cursor_style(app));

    frame.render_widget(&search_bar, area);
}
