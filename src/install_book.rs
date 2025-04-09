use std::sync::Arc;

use crate::{
    app::{App, DownloadStatus},
    download::download_book,
    download_url::return_download_url,
};

pub async fn install_book(app: &mut App) {
    if let Some(selected) = app.table_state.selected() {
        let selected_book = app.search_results[selected].clone();
        let title = selected_book.title.clone();
        let title_formatted = title.clone().replace(" ", "_");
        let md5 = selected_book.md5.clone();
        let client_clone = app.client.clone();
        let extension = selected_book.extension.clone();
        let download_dir = app.config.download_directory.clone();

        let downloads = Arc::clone(&app.downloads);
        app.downloads
            .lock()
            .unwrap()
            .insert((title.clone(), md5.clone()), DownloadStatus::Pending);

        tokio::spawn(async move {
            match return_download_url(md5.clone(), client_clone).await {
                Ok(url) => {
                    let filename = format!("{}.{}", title_formatted, extension);

                    let destination = if download_dir.ends_with("/") {
                        format!("{}{}", download_dir, filename)
                    } else {
                        format!("{}/{}", download_dir, filename)
                    };

                    if let Err(e) = download_book(&url, &destination).await {
                        eprintln!("Error downloading book: {}", e);
                    } else {
                        downloads
                            .lock()
                            .unwrap()
                            .insert((title.clone(), md5.clone()), DownloadStatus::Completed);
                    }
                }
                Err(_) => {
                    downloads
                        .lock()
                        .unwrap()
                        .insert((title.clone(), md5.clone()), DownloadStatus::Failed);
                }
            }
        });
    }
}
