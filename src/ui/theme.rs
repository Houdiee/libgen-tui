use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders},
};

use crate::app::{App, Focus};

pub fn border_style(app: &App, target: Focus) -> Style {
    if app.focus != target {
        return Style::new().white();
    }

    match target {
        Focus::PopupYes => Style::new().green(),
        Focus::PopupCancel => Style::new().red(),
        _ => Style::new().blue(),
    }
}

pub fn pane(title: &str, style: Style) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(style)
        .title(title.to_string())
}
