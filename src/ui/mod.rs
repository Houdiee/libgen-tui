pub mod downloads;
pub mod popup;
pub mod results;
pub mod search_bar;
pub mod theme;

use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let [search_area, results_area, downloads_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Percentage(70),
        Constraint::Percentage(30),
    ])
    .areas(frame.area());

    search_bar::render(frame, app, search_area);
    results::render(frame, app, results_area);
    downloads::render(frame, app, downloads_area);

    if app.show_popup {
        popup::render(frame, app);
    }
}
