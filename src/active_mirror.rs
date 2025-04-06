use colored::Colorize;
use config::Config;
use futures::{future::select_all, FutureExt};
use log::info;
use reqwest::{Client, Response};

pub async fn fetch_mirror(client: Client, url: String) -> Result<Response, reqwest::Error> {
    info!("Testing connection to {}", url);
    client.get(url).send().await
}

#[derive(Debug)]
pub enum Error {
    NoActiveMirror,
}

pub async fn check_mirrors_and_return_active(
    client: Client,
    mirrors: Vec<String>,
) -> Result<String, Error> {
    println!("{}", "Attempting to connect to libgen mirrors...".yellow());

    let futures: Vec<_> = mirrors
        .clone()
        .into_iter()
        .map(|mirror| {
            let url = format!("https://{}/", mirror);
            fetch_mirror(client.clone(), url).boxed()
        })
        .collect();

    let mut remaining_futures = futures;

    while !remaining_futures.is_empty() {
        let (result, index, remaining) = select_all(remaining_futures).await;

        if result.is_ok() {
            println!("{}", "Connected to mirror!".green());
            return Ok(mirrors[mirrors.len() - remaining.len() - 1 + index].clone());
        } else {
            remaining_futures = remaining;
        }
    }

    println!(
        "{}",
        "Failed to connect to mirrors. Is the mirror accessible? (Note: only secure http protocol is allowed)".red()
    );
    Err(Error::NoActiveMirror)
}
