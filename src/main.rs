use crate::cli::arg_config;
mod banner;
mod cli;
mod checker;
mod scraper;
mod writer;
mod progress;
use writer::file::writeinfile;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    banner::print_banner();
    
    if arg_config().output_file {
        writeinfile().await?;
    } else {
        checker::client::check_all().await;
    }

    Ok(())
}