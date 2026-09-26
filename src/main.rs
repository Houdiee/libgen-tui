use colored::Colorize;

use libgen_tui::app::config::default_mirrors;
use libgen_tui::app::{App, AppConfig};
use libgen_tui::libgen::mirror;
use libgen_tui::run::run;

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off")).init();

    let mut app = App::new(AppConfig::load());

    let builtin = default_mirrors();
    let additional = app.config.additional_mirrors.clone();

    println!("{}", "Attempting to connect to libgen mirrors...".yellow());

    let candidates = mirror::merge(&[&builtin, &additional]);

    let Some(active) = mirror::find_active(&app.client, &candidates).await else {
        eprintln!(
            "{}\nTried: {}\nAdd working domains to additional_mirrors in {}",
            "Failed to connect to any libgen mirror.".red(),
            candidates.join(", "),
            AppConfig::path().display(),
        );
        return;
    };

    println!("{} {}", "Connected to mirror:".green(), active.host.green());

    app.mirrors = mirror::merge(&[&active.siblings, &builtin, &additional]);
    app.active_mirror = Some(active.host);

    let terminal = ratatui::init();
    run(terminal, &mut app).await;
    ratatui::restore();
}
