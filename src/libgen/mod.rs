pub mod download;
pub mod mirror;
pub mod search;

use std::time::Duration;

use reqwest::Client;

pub use search::Book;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub fn build_client() -> Client {
    Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .expect("Failed to build http client.")
}

pub fn mirrors_by_preference(mirrors: &[String], preferred: &str) -> Vec<String> {
    let mut ordered = vec![preferred.to_string()];
    ordered.extend(
        mirrors
            .iter()
            .filter(|mirror| mirror.as_str() != preferred)
            .cloned(),
    );
    ordered
}
