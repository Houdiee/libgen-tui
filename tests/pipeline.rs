use libgen_tui::app::config::DEFAULT_MIRRORS;
use libgen_tui::libgen::{build_client, download, mirror, search};

#[tokio::test]
async fn pipeline_works_end_to_end() {
    let client = build_client();
    let mirrors: Vec<String> = DEFAULT_MIRRORS.iter().map(|m| m.to_string()).collect();

    let active = mirror::find_active(&client, &mirrors)
        .await
        .expect("no mirror is reachable");
    println!("mirror: {}", active);

    let (books, served_by) = search::search(&client, &mirrors, &active, "rust programming", 25)
        .await
        .expect("search failed");
    assert!(!books.is_empty(), "search returned no books");

    let book = &books[0];
    println!(
        "{} books from {}; first: {:?} ({} {}) md5={}",
        books.len(),
        served_by,
        book.title,
        book.size,
        book.extension,
        book.md5
    );

    assert!(!book.title.is_empty(), "title was not parsed");
    assert_eq!(book.md5.len(), 32, "md5 was not parsed: {:?}", book.md5);
    assert!(!book.extension.is_empty(), "extension was not parsed");

    let url = download::resolve_url_with_failover(&client, &mirrors, &served_by, &book.md5)
        .await
        .expect("could not resolve a download url");
    assert!(url.contains("key="), "download url carries no key: {}", url);

    let destination = std::env::temp_dir().join("libgen-tui-pipeline-test.bin");
    download::download_to_file(&client, &url, &destination)
        .await
        .expect("download failed");

    let written = std::fs::metadata(&destination)
        .expect("file was not written")
        .len();
    println!("downloaded {} bytes", written);
    assert!(written > 1024, "download was suspiciously small");

    let _ = std::fs::remove_file(&destination);
}
