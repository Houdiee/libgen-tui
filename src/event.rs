use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Focus, ResultsState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    None,
    Search(String),
    Download,
}

pub const MIN_QUERY_LEN: usize = 2;

fn is_interrupt(key: KeyEvent) -> bool {
    key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c'))
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Action {
    if is_interrupt(key) {
        app.should_quit = true;
        return Action::None;
    }

    match app.focus {
        Focus::SearchBar => search_bar_key(app, key),
        Focus::Nothing => nothing_key(app, key),
        Focus::Table => table_key(app, key),
        Focus::PopupYes => popup_yes_key(app, key),
        Focus::PopupCancel => popup_cancel_key(app, key),
    }
}

fn search_bar_key(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => app.focus = Focus::Nothing,

        KeyCode::Tab => {
            if !app.search_results.is_empty() {
                app.focus = Focus::Table;
            }
        }

        KeyCode::Enter => {
            let query = app.query();

            if query.len() < MIN_QUERY_LEN {
                app.results_state = ResultsState::QueryTooShort;
                return Action::None;
            }

            app.results_state = ResultsState::Searching;
            app.focus = Focus::Table;
            return Action::Search(query);
        }

        _ => {
            app.search_bar.input(key);
        }
    }

    Action::None
}

fn nothing_key(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Tab | KeyCode::Char('/') => app.focus = Focus::SearchBar,
        KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Down | KeyCode::Up => {
            app.focus = Focus::Table
        }
        _ => {}
    }

    Action::None
}

fn table_key(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Tab | KeyCode::Char('/') => app.focus = Focus::SearchBar,
        KeyCode::Esc => app.focus = Focus::Nothing,

        KeyCode::Char('j') | KeyCode::Down => app.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.select_previous(),

        KeyCode::Char('g') => {
            if app.table_state.selected().is_some() {
                app.table_state.select_first();
            }
        }
        KeyCode::Char('G') => {
            if app.table_state.selected().is_some() {
                app.table_state.select_last();
            }
        }

        KeyCode::Enter => {
            if app.table_state.selected().is_some() {
                app.show_popup = true;
                app.focus = Focus::PopupYes;
            }
        }

        KeyCode::Char(' ') if app.table_state.selected().is_some() => return Action::Download,

        _ => {}
    }

    Action::None
}

fn popup_yes_key(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Tab | KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('j') => {
            app.focus = Focus::PopupCancel
        }
        KeyCode::Esc | KeyCode::Char('q') => close_popup(app),
        KeyCode::Enter => {
            close_popup(app);
            return Action::Download;
        }
        _ => {}
    }

    Action::None
}

fn popup_cancel_key(app: &mut App, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('k') => {
            app.focus = Focus::PopupYes
        }
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => close_popup(app),
        _ => {}
    }

    Action::None
}

fn close_popup(app: &mut App) {
    app.show_popup = false;
    app.focus = Focus::Table;
}
