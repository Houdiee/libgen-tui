use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, Focus};
use crate::ui::theme;

const MAX_WIDTH: u16 = 64;
const BUTTON_HEIGHT: u16 = 3;
const VERTICAL_PADDING: u16 = 1;
const HORIZONTAL_PADDING: u16 = 2;

fn wrap(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            current = word.to_string();
        } else if current.chars().count() + 1 + word.chars().count() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);

    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}

fn button(label: &'static str, color: Color, border: Style) -> Paragraph<'static> {
    Paragraph::new(label)
        .centered()
        .style(Style::default().fg(color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border),
        )
}

pub fn render(frame: &mut Frame, app: &App) {
    let Some(book) = app.selected_book() else {
        return;
    };

    let message = format!(
        "Confirm installation for '{}' by '{}'?",
        book.title, book.author
    );

    let screen = frame.area();
    let outer_width = MAX_WIDTH.min(screen.width.saturating_sub(4)).max(20);
    let text_width = outer_width.saturating_sub(2 + HORIZONTAL_PADDING * 2);
    let lines = wrap(&message, text_width as usize);

    let outer_height = lines.len() as u16 + VERTICAL_PADDING * 2 + BUTTON_HEIGHT + 2;
    let area = centered_rect(screen, outer_width, outer_height);

    let frame_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue())
        .title(Line::from(" Confirm installation ").centered());

    let inner = frame_block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(frame_block, area);

    let [_, text_area, _, buttons_area] = Layout::vertical([
        Constraint::Length(VERTICAL_PADDING),
        Constraint::Length(lines.len() as u16),
        Constraint::Length(VERTICAL_PADDING),
        Constraint::Length(BUTTON_HEIGHT),
    ])
    .horizontal_margin(HORIZONTAL_PADDING)
    .areas(inner);

    let text = Paragraph::new(
        lines
            .into_iter()
            .map(|line| Line::from(line).centered())
            .collect::<Vec<_>>(),
    );
    frame.render_widget(text, text_area);

    let [cancel_area, install_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .spacing(HORIZONTAL_PADDING)
            .areas(buttons_area);

    frame.render_widget(
        button(
            "Cancel",
            Color::Red,
            theme::border_style(app, Focus::PopupCancel),
        ),
        cancel_area,
    );
    frame.render_widget(
        button(
            "Install",
            Color::Green,
            theme::border_style(app, Focus::PopupYes),
        ),
        install_area,
    );
}
