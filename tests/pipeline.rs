use libgen_tui::app::config::default_mirrors;
use libgen_tui::libgen::{build_client, download, mirror, search};

#[tokio::test]
async fn pipeline_works_end_to_end() {
    let client = build_client();
    let builtin = default_mirrors();

    let active = mirror::find_active(&client, &builtin)
        .await
        .expect("no mirror is reachable");
    println!("mirror: {}", active.host);
    println!("discovered siblings: {:?}", active.siblings);

    assert!(
        !active.siblings.is_empty(),
        "{} advertised no sibling domains, so mirror discovery has broken",
        active.host
    );

    let mirrors = mirror::merge(&[&active.siblings, &builtin]);
    println!("session mirror list: {:?}", mirrors);

    let (books, served_by) =
        search::search(&client, &mirrors, &active.host, "rust programming", 25)
            .await
            .expect("search failed");
    assert!(!books.is_empty(), "search returned no books");
    println!("{} books from {}", books.len(), served_by);

    let book = &books[0];
    assert!(!book.title.is_empty(), "title was not parsed");
    assert_eq!(book.md5.len(), 32, "md5 was not parsed: {:?}", book.md5);
    assert!(!book.extension.is_empty(), "extension was not parsed");

    let populated = |field: fn(&libgen_tui::libgen::Book) -> &String| {
        books.iter().filter(|b| !field(b).is_empty()).count()
    };
    for (name, count) in [
        ("author", populated(|b| &b.author)),
        ("year", populated(|b| &b.year)),
        ("languages", populated(|b| &b.languages)),
        ("size", populated(|b| &b.size)),
        ("extension", populated(|b| &b.extension)),
    ] {
        assert!(
            count > books.len() / 2,
            "column {} resolved for only {}/{} rows, header mapping is off",
            name,
            count,
            books.len()
        );
    }

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
