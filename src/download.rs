use std::error;

use tokio::{fs::File, io::AsyncWriteExt};

pub async fn download_book(
    download_url: &str,
    destination: &str,
) -> Result<(), Box<dyn error::Error>> {
    let response = reqwest::get(download_url).await?;
    let mut file = File::create(destination).await?;
    let content = response.bytes().await?;
    file.write(&content).await?;
    Ok(())
}
