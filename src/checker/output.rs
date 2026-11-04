use crate::checker::client::FinalResult;
use std::sync::OnceLock;
use terminal_size::{terminal_size, Width as TermWidth};

const W_NO: usize = 4;
const W_KEY: usize = 10;
const W_ACTIVE: usize = 25;
const W_AGO: usize = 22;
const W_STATUS: usize = 6;

const YELLOW: &str = "1;38;5;221";
const CYAN: &str = "1;38;5;87";
const PEACH: &str = "1;38;5;216";
const ORANGE: &str = "1;38;5;208";
const PURPLE: &str = "1;38;5;141";
const RED: &str = "1;38;5;203";
const MAGENTA: &str = "1;38;5;207";
const WHITE: &str = "1;38;5;255";

fn term_width() -> usize {
    terminal_size()
        .map(|(TermWidth(w), _)| w as usize)
        .unwrap_or(120)
}

fn link_width() -> usize {
    static W: OnceLock<usize> = OnceLock::new();
    *W.get_or_init(|| {
        let fixed = W_NO + W_KEY + W_ACTIVE + W_AGO + W_STATUS + 12 + 7;
        term_width().saturating_sub(fixed).clamp(20, 70)
    })
}

fn paint(text: &str, code: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", code, text)
}

fn fit(text: &str, width: usize) -> String {
    let len = text.chars().count();
    if len > width {
        let cut: String = text.chars().take(width.saturating_sub(2)).collect();
        format!("{}..", cut)
    } else {
        format!("{}{}", text, " ".repeat(width - len))
    }
}

fn border(left: &str, mid: &str, right: &str) -> String {
    let widths = [W_NO, W_KEY, link_width(), W_ACTIVE, W_AGO, W_STATUS];
    let parts: Vec<String> = widths.iter().map(|w| "─".repeat(w + 2)).collect();
    format!("{}{}{}", left, parts.join(mid), right)
}

fn line(cells: [String; 6]) -> String {
    format!("│ {} │", cells.join(" │ "))
}

pub fn print_header() {
    println!("{}", border("╭", "┬", "╮"));
    println!(
        "{}",
        line([
            paint(&fit("No", W_NO), PURPLE),
            paint(&fit("Keyword", W_KEY), ORANGE),
            paint(&fit("Link", link_width()), YELLOW),
            paint(&fit("Last Active", W_ACTIVE), MAGENTA),
            paint(&fit("Time Ago", W_AGO), PEACH),
            paint(&fit("Status", W_STATUS), RED),
        ])
    );
    println!("{}", border("├", "┼", "┤"));
}

pub fn print_result(r: &FinalResult) {
    println!(
        "{}",
        line([
            paint(&fit(&r.site_number.to_string(), W_NO), ORANGE),
            paint(&fit(&r.keyword, W_KEY), WHITE),
            paint(&fit(&r.link, link_width()), PEACH),
            paint(&fit(&r.last_active, W_ACTIVE), MAGENTA),
            paint(&fit(&r.time_ago, W_AGO), CYAN),
            paint(&fit(&r.status_code.to_string(), W_STATUS), YELLOW),
        ])
    );
}

pub fn print_footer() {
    println!("{}", border("╰", "┴", "╯"));
}