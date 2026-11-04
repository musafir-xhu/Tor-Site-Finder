use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use hypertor::TorClient;
use crate::cli::arg_config;
use indicatif::ProgressBar;
use crate::progress::spinner;
use crate::checker::output;
use crate::scraper::parser::{self, ExtractedResult};
use futures::stream::{self, StreamExt};
use tabled::Tabled;
use serde::Serialize;

#[derive(Tabled, Debug, Serialize)]
pub struct FinalResult {
    #[tabled(rename = "No")]
    pub site_number: u64,
    #[tabled(rename = "Keyword")]
    pub keyword: String,
    #[tabled(rename = "Link")]
    pub link: String,
    #[tabled(rename = "Last Active")]
    pub last_active: String,
    #[tabled(rename = "Time Ago")]
    pub time_ago: String,
    #[tabled(rename = "Status")]
    pub status_code: u16,
}

pub async fn client() -> hypertor::Result<TorClient> {
    TorClient::builder()
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(30))
        .build()
        .await
}

async fn check_one(
    client: &TorClient,
    item: ExtractedResult,
    ts: &AtomicU64,
    wanted: &Option<Vec<u16>>,
    live: bool,
) -> Option<FinalResult> {
    let host = item
        .url
        .trim_start_matches("http://")
        .trim_start_matches("https://");

    for scheme in ["http", "https"] {
        let full_url = format!("{}://{}", scheme, host);

        let req = match client.get(full_url.as_str()) {
            Ok(r) => r,
            Err(_) => continue,
        };

        match req.send().await {
            Ok(resp) => {
                let code = resp.status().as_u16();

                if let Some(list) = wanted {
                    if !list.contains(&code) {
                        return None;
                    }
                }

                let result = FinalResult {
                    site_number: ts.fetch_add(1, Ordering::SeqCst) + 1,
                    keyword: item.keyword.clone(),
                    link: full_url,
                    last_active: item.last_active.clone(),
                    time_ago: item.time_ago.clone(),
                    status_code: code,
                };

                if live {
                    output::print_result(&result);
                }

                return Some(result);
            }
            Err(e) => {
                if e.to_string().contains("timed out") {
                    break;
                }
            }
        }
    }

    None
}

pub async fn check_all() -> Vec<FinalResult> {
    let client = client().await.expect("Error: Failed to build Tor client");
    let client = &client;
    let items = parser::extract_urls().await;

    let ts = AtomicU64::new(0);
    let ts = &ts;

    let wanted = arg_config().status_code.clone();
    let wanted = &wanted;
    let live = !arg_config().output_file;

    let sp = if !live && arg_config().verbose {
        spinner::start("Checking onion links status...")
    } else {
        ProgressBar::hidden()
    };

    if live {
        output::print_header();
    }

    let stream = stream::iter(items)
        .map(|item| async move { check_one(client, item, ts, wanted, live).await })
        .buffer_unordered(50);
    tokio::pin!(stream);

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    let mut results = Vec::new();
    loop {
        tokio::select! {
            next = stream.next() => {
                match next {
                    Some(Some(r)) => results.push(r),
                    Some(None) => {}
                    None => break,
                }
            }
            _ = &mut ctrl_c => break,
        }
    }

    if live {
        output::print_footer();
    }

    spinner::finish(&sp, &format!("Checked onion links: {} responded", results.len()));

    results
}