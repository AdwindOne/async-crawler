use rusqlite::{params, Connection};
use scraper::{Html, Selector};
use reqwest;
use url::Url;

#[allow(dead_code)]
pub fn init_db(path: &str) -> Connection {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS pages (
            url TEXT PRIMARY KEY,
            title TEXT,
            body TEXT,
            keyword TEXT
        );
        CREATE TABLE IF NOT EXISTS links (
            from_url TEXT,
            to_url TEXT
        );
    ").unwrap();
    conn
}

pub async fn fetch_page(url: &str) -> Option<(String, String, Vec<String>)> {
    let res = reqwest::get(url).await.ok()?.text().await.ok()?;

    let document = Html::parse_document(&res);
    let title = document.select(&Selector::parse("title").unwrap())
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or_default();

    let selector = Selector::parse("a").unwrap();
    let base = Url::parse(url).ok()?;

    let mut links = Vec::new();
    for a in document.select(&selector) {
        if let Some(href) = a.value().attr("href") {
            if let Ok(abs) = base.join(href) {
                links.push(abs.to_string());
            }
        }
    }

    Some((title, res, links))
}


pub fn creat_table() -> rusqlite::Result<()> {
    let conn = Connection::open("data.db")?;
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS pages (
            url TEXT PRIMARY KEY,
            title TEXT,
            body TEXT,
            keyword TEXT
        );
        CREATE TABLE IF NOT EXISTS links (
            from_url TEXT,
            to_url TEXT
        );
    ")?;
    Ok(())
}
pub fn save_page(url: &str, title: &str, body: &str, keyword: &str) -> rusqlite::Result<()> {
    let conn = Connection::open("data.db")?;
    conn.execute(
        "INSERT OR IGNORE INTO pages (url, title, body, keyword) VALUES (?1, ?2, ?3, ?4)",
        params![url, title, body, keyword],
    )?;
    Ok(())
}

pub fn save_links(from_url: &str, to_urls: &Vec<String>) -> rusqlite::Result<()> {
    let conn = Connection::open("data.db")?;
    for to_url in to_urls {
        conn.execute(
            "INSERT OR IGNORE INTO links (from_url, to_url) VALUES (?1, ?2)",
            params![from_url, to_url],
        )?;
    }
    Ok(())
}