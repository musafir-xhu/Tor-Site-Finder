use reqwest::Client; // async client instead of reqwest::blocking::Client
use crate::{cli, progress::spinner};
use scraper::{ElementRef, Html, Selector};
use cli::arg_config;
use std::collections::HashSet;
use indicatif::ProgressBar;

#[derive(Debug)]
pub struct ExtractedResult {
    pub keyword: String,
    pub url: String,
    pub last_active: String,
    pub time_ago: String,
}

pub fn client() -> Client {
    Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0")
        .build()
        .expect("Error: Failed to initialize HTTP client")
}

pub fn text_of(parent: &ElementRef, sel: &Selector) -> String {
    parent
        .select(sel)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn attr_of(parent: &ElementRef, sel: &Selector, attr: &str) -> String {
    parent
        .select(sel)
        .next()
        .and_then(|e| e.value().attr(attr))
        .unwrap_or_default()
        .to_string()
}

// sync helper: parse the hidden form token out of the HTML (no await inside)
fn parse_token(html: &str) -> Option<(String, String)> {
    let doc = Html::parse_document(html);
    let hidden = Selector::parse("form input[type=hidden]")
        .expect("Error: Invalid CSS selector for hidden form inputs");

    doc.select(&hidden).find_map(|ex| {
        match (ex.value().attr("name"), ex.value().attr("value")) {
            (Some(name), Some(value)) => Some((name.to_string(), value.to_string())),
            _ => None,
        }
    })
}

fn parse_results(html: &str, key: &str) -> Vec<ExtractedResult> {
    let document = Html::parse_document(html);
    let sell1 = Selector::parse("li.result").unwrap();
    let selector_url = Selector::parse("cite").unwrap();
    let selector_timestamp = Selector::parse("span.lastSeen").unwrap();

    let mut out = Vec::new();
    for elem in document.select(&sell1) {
        let link = text_of(&elem, &selector_url);
        let last_seen = attr_of(&elem, &selector_timestamp, "data-timestamp");
        let ago = text_of(&elem, &selector_timestamp).replace('\u{a0}', " ");

        out.push(ExtractedResult {
            keyword: key.to_string(),
            url: link,
            last_active: last_seen,
            time_ago: ago,
        });
    }
    out
}

pub async fn token_extract(client: &Client) -> Option<(String, String)> {
    let sp = if cli::arg_config().verbose {
        spinner::start("Extracting search token from form...")
    } else {
        ProgressBar::hidden()
    };

    let response = client
        .get("https://ahmia.fi/search")
        .send()
        .await // async: wait for the response
        .expect("Error: Failed to send HTTP request to target site")
        .text()
        .await // async: wait for the body
        .expect("Error: Failed to read response body as text");

    let token = parse_token(&response);

    spinner::finish(&sp, "Search token extracted");
    token
}

pub async fn extract_urls() -> Vec<ExtractedResult> {
    let client = client();
    let token = token_extract(&client).await; // reuse the same client so cookies are kept
    let mut extracted_links: Vec<ExtractedResult> = Vec::new();

    let sp = if cli::arg_config().verbose {
        spinner::start("Extracting onion links from results...")
    } else {
        ProgressBar::hidden()
    };

    for key in arg_config().keyword {
        let mut param = vec![("q".to_string(), key.clone())];
        if let Some(tok) = &token {
            param.push(tok.clone());
        }

        let response = client
            .get("https://ahmia.fi/search")
            .query(&param)
            .send()
            .await
            .expect("Error: Failed to send HTTP request to target site")
            .text()
            .await
            .expect("Error: Failed to read response body as text");

        extracted_links.extend(parse_results(&response, &key));
    }

    spinner::finish(&sp, "Onion links extracted");
    checktotal(&extracted_links);
    unique_links(extracted_links)
}

pub fn unique_links(results: Vec<ExtractedResult>) -> Vec<ExtractedResult> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut unique: Vec<ExtractedResult> = Vec::new();

    for r in results {
        if seen.insert(r.url.clone()) {
            unique.push(r);
        }
    }

    unique
}

fn checktotal(extracted_links: &Vec<ExtractedResult>) {
    let sp = if cli::arg_config().verbose {
        spinner::start("Checking total number of onion links...")
    } else {
        ProgressBar::hidden()
    };

    let total = extracted_links.len();
    let tfm = format!("Total number of onion link: {}", total);
    spinner::finish(&sp, &tfm);

    let sp2 = if cli::arg_config().verbose {
        spinner::start("Checking total number of duplicates onion links...")
    } else {
        ProgressBar::hidden()
    };
    let unique: HashSet<&String> = extracted_links.iter().map(|r| &r.url).collect();
    let duplicates = total - unique.len();

    let tfm2 = format!("Total number of duplicates onion link: {}", duplicates);
    spinner::finish(&sp2, &tfm2);
}