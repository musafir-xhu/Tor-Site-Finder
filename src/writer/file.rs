use crate::cli::arg_config;
use crate::checker::client;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use indicatif::ProgressBar;
use crate::progress::spinner;

fn outputfile() -> Vec<String> {
    let mut filenames: Vec<String> = Vec::new();

    match arg_config().file_format {
        Some(val) => {
            for ex in val {
                let f = format!("tor_sites.{}", ex);
                filenames.push(f);
            }
        }
        None => {
            filenames.push("tor_sites.txt".to_string());
        }
    }

    filenames
}

pub async fn writeinfile() -> Result<(), Box<dyn Error>> {
    if arg_config().output_file {
        let all_data = client::check_all().await;

        let sp = if arg_config().verbose {
            spinner::start("Writing scraping results to file...")
        } else {
            ProgressBar::hidden()
        };

        for filename in outputfile() {
            if filename.ends_with(".txt") {
                let mut file = File::create(&filename)?;

                for item in &all_data {
                    writeln!(file, "Site Number: {}", item.site_number)?;
                    writeln!(file, "Keyword: {}", item.keyword)?;
                    writeln!(file, "Link: {}", item.link)?;
                    writeln!(file, "Last Active: {}", item.last_active)?;
                    writeln!(file, "Time Ago: {}", item.time_ago)?;
                    writeln!(file, "Status Code: {}", item.status_code)?;
                    writeln!(file)?;
                }
            } else if filename.ends_with(".csv") {
                let mut writer = csv::Writer::from_path(&filename)?;

                for item in &all_data {
                    writer.serialize(item)?;
                }

                writer.flush()?;
            } else if filename.ends_with(".json") {
                let file = File::create(&filename)?;

                serde_json::to_writer_pretty(file, &all_data)?;
            }
        }

        spinner::finish(&sp, "Scraping results successfully written to file.");
    }

    Ok(())
}