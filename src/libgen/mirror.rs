use futures::{future::select_all, FutureExt};
use log::info;
use reqwest::Client;

async fn probe(client: Client, mirror: String) -> Result<String, ()> {
    let url = format!("https://{}/", mirror);
    info!("Testing connection to {}", url);

    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => Ok(mirror),
        Ok(response) => {
            info!("{} responded with {}", mirror, response.status());
            Err(())
        }
        Err(e) => {
            info!("{} failed: {}", mirror, e);
            Err(())
        }
    }
}

pub async fn find_active(client: &Client, mirrors: &[String]) -> Option<String> {
    let mut remaining: Vec<_> = mirrors
        .iter()
        .map(|mirror| probe(client.clone(), mirror.clone()).boxed())
        .collect();

    while !remaining.is_empty() {
        let (result, _index, rest) = select_all(remaining).await;

        match result {
            Ok(mirror) => return Some(mirror),
            Err(()) => remaining = rest,
        }
    }

    None
}
