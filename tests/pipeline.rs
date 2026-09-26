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
    println!("{} books from {}", books.len(), served_by);

    let book = &books[0];
    assert!(!book.title.is_empty(), "title was not parsed");
    assert_eq!(book.md5.len(), 32, "md5 was not parsed: {:?}", book.md5);
    assert!(!book.extension.is_empty(), "extension was not parsed");

    let destination = std::env::temp_dir().join("libgen-tui-pipeline-test.bin");
    let mut failures = Vec::new();

    for book in books.iter().take(5) {
        let url =
            match download::resolve_url_with_failover(&client, &mirrors, &served_by, &book.md5)
                .await
            {
                Ok(url) => url,
                Err(e) => {
                    failures.push(format!("{}: {}", book.title, e));
                    continue;
                }
            };
        assert!(url.contains("key="), "download url carries no key: {}", url);

        match download::download_to_file(&client, &url, &destination).await {
            Ok(()) => {
                let written = std::fs::metadata(&destination)
                    .expect("file was not written")
                    .len();
                println!(
                    "downloaded {:?}: {} bytes from {}",
                    book.title, written, url
                );
                assert!(written > 1024, "download was suspiciously small");
                let _ = std::fs::remove_file(&destination);
                return;
            }
            Err(e) => failures.push(format!("{}: {}", book.title, e)),
        }
    }

    panic!(
        "every download failed. A 5xx here is a libgen CDN outage rather than a \
         problem with this crate; anything else is worth investigating:\n  {}",
        failures.join("\n  ")
    );
}
