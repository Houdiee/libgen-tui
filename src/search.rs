use reqwest::Client;
use scraper::{Html, Selector};
use urlencoding::encode;

use crate::app::Book;

pub async fn return_books_from_search(
    mirror: &str,
    query: &str,
    client: Client,
) -> Result<Vec<Book>, reqwest::Error> {
    let encode = encode(query);
    let url = format!("https://{}/search.php?req={}", mirror, encode);
    let body = client.get(url).send().await?.text().await?;

    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table.c").unwrap();
    let row_selector = Selector::parse("tr[bgcolor]").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    let anchor_selector = Selector::parse("a").unwrap();
    let italic_selector = Selector::parse("i").unwrap();

    let mut books: Vec<Book> = Vec::new();

    for table in document.select(&table_selector) {
        for row in table.select(&row_selector) {
            let mut cells: Vec<String> = row
                .select(&cell_selector)
                .map(|c| {
                    let mut text = c.text().collect::<String>();

                    for italic in c.select(&italic_selector) {
                        text = text.replace(&italic.text().collect::<String>(), "");
                    }

                    text.trim().to_string()
                })
                .take(9)
                .collect();

            if cells.len() == 9 {
                let mut book = Book {
                    id: cells.remove(0),
                    author: cells.remove(0),
                    title: cells.remove(0),
                    publisher: cells.remove(0),
                    year: cells.remove(0),
                    pages: cells.remove(0),
                    languages: cells.remove(0),
                    size: cells.remove(0),
                    extension: cells.remove(0),
                    md5: String::new(),
                };

                if let Some(anchor) = row
                    .select(&cell_selector)
                    .nth(2)
                    .and_then(|c| c.select(&anchor_selector).next())
                {
                    if anchor.value().attr("title").is_some() {
                        if let Some(href) = anchor.value().attr("href") {
                            if let Some(equals_pos) = href.find('=') {
                                book.md5 = href[equals_pos + 1..].to_string();
                            }
                        }
                    }
                }
                books.push(book);
            }
        }
    }

    assert!(!books.is_empty());
    if !books.is_empty() {
        books.remove(0);
    }

    Ok(books)
}
