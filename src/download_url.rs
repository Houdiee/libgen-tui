use reqwest::Client;
use scraper::{selectable::Selectable, Html, Selector};

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum DownloadUrlError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("anchor tag not found")]
    LinkNotFound,
    #[error("download URL not found")]
    Failed,
}

pub async fn return_download_url(md5: String, client: Client) -> Result<String, DownloadUrlError> {
    let url = format!("https://books.ms/main/{}", md5);
    let body = client.get(url).send().await?.text().await?;

    let document = Html::parse_document(&body);
    let div_selector = Selector::parse("div#download").unwrap();
    let h2_selector = Selector::parse("h2").unwrap();
    let anchor_selector = Selector::parse("a").unwrap();

    for div in document.select(&div_selector) {
        for h2 in div.select(&h2_selector) {
            for anchor in h2.select(&anchor_selector) {
                if let Some(href) = anchor.value().attr("href") {
                    let download_url = href.to_string();
                    return Ok(download_url);
                } else {
                    return Err(DownloadUrlError::LinkNotFound);
                }
            }
        }
    }

    return Err(DownloadUrlError::Failed);
}
