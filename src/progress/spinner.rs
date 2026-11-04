use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn start(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.yellow.bright.bold} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✔"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

pub fn finish(pb: &ProgressBar, done_msg: &str) {
    pb.set_style(
        ProgressStyle::with_template("{prefix:.yellow.bright.bold} {msg:.216.bold}").unwrap(),
    );
    pb.set_prefix("✔");
    pb.finish_with_message(done_msg.to_string());
}

