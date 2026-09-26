use std::time::Duration;

use ratatui::{
    crossterm::event::{self, poll, Event, KeyEventKind},
    DefaultTerminal,
};

use crate::app::{App, Focus, ResultsState};
use crate::event::{handle_key, Action};
use crate::libgen::search;
use crate::ui;

const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub async fn run(mut terminal: DefaultTerminal, app: &mut App) {
    loop {
        terminal
            .draw(|frame| ui::draw(frame, app))
            .expect("Failed to draw to terminal.");

        if !poll(POLL_INTERVAL).expect("Failed to poll.") {
            continue;
        }

        let Event::Key(key) = event::read().expect("Failed to read event.") else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        match handle_key(app, key) {
            Action::None => {}
            Action::Search(query) => {
                terminal
                    .draw(|frame| ui::draw(frame, app))
                    .expect("Failed to draw to terminal.");

                perform_search(app, &query).await;
            }
            Action::Download => {
                app.start_selected_download();
                app.select_next();
            }
        }

        if app.should_quit {
            break;
        }
    }
}

async fn perform_search(app: &mut App, query: &str) {
    let Some(mirror) = app.active_mirror.clone() else {
        app.results_state = ResultsState::Failed("no mirror is reachable".to_string());
        app.focus = Focus::SearchBar;
        return;
    };

    let result = search::search(
        &app.client,
        &app.mirrors,
        &mirror,
        query,
        app.config.max_results,
    )
    .await;

    match result {
        Ok((results, served_by)) => {
            app.active_mirror = Some(served_by);
            app.set_results(results);
        }
        Err(e) => {
            log::error!("Search failed on all mirrors: {}", e);
            app.results_state = ResultsState::Failed(e.to_string());
            app.focus = Focus::SearchBar;
        }
    }
}
