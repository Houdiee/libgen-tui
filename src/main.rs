use colored::Colorize;

use libgen_tui::app::{App, AppConfig};
use libgen_tui::libgen::mirror;
use libgen_tui::run::run;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off")).init();

    let mut app = App::new(AppConfig::load());

    println!("{}", "Attempting to connect to libgen mirrors...".yellow());

    match mirror::find_active(&app.client, &app.config.mirrors).await {
        Some(found) => {
            println!("{} {}", "Connected to mirror:".green(), found.green());
            app.active_mirror = Some(found);
        }
        None => {
            eprintln!(
                "{}\nTried: {}\nEdit the mirror list at {}\nKnown working mirrors: {}",
                "Failed to connect to any libgen mirror.".red(),
                app.config.mirrors.join(", "),
                AppConfig::path().display(),
                libgen_tui::app::config::DEFAULT_MIRRORS.join(", "),
            );
            return;
        }
    }

    let terminal = ratatui::init();
    run(terminal, &mut app).await;
    ratatui::restore();
}
