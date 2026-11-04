# Tor Site Finder

Rust CLI that finds Tor .onion links by keyword and checks if they are online

<p align="center">
  <img src="https://github.com/user-attachments/assets/d6e578e0-cad9-437b-9c3b-f6eb4bbd6686" alt="Tor Link Finder" width="100%">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Tor-7D4698?logo=torproject&logoColor=white" alt="Tor">
  <img src="https://img.shields.io/badge/async-tokio-5fffff" alt="Async">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-ffd75f.svg" alt="MIT License"></a>
</p>

<p align="center">
  <b>Search onion links by keyword, then check which ones are alive, all from your terminal.</b>
</p>

---

## What is Tor Link Finder?

**Tor Link Finder** is a fast command-line tool written in Rust. It searches the [Ahmia](https://ahmia.fi) search engine for `.onion` links that match your keywords, then visits every link over Tor to see which ones still respond. The results appear live in a colored table, and you can save them to a file.

## Features

- **Keyword search**: finds onion links through Ahmia for the keywords you give it
- **Status check**: visits each link over Tor and reports its HTTP status code
- **Built-in Tor**: no separate Tor install or daemon is needed, because the tool uses the embedded [Arti](https://gitlab.torproject.org/tpo/core/arti) client through the `hypertor` crate
- **Very fast**: checks up to 50 links at the same time with async Rust
- **Smart retries**: tries `http://` first and falls back to `https://`
- **Status filter**: keep only the status codes you care about
- **Last active info**: shows when Ahmia last saw each site online, and how long ago that was
- **Duplicate removal**: the same link found twice is only checked once
- **Save results**: export to `.txt`, `.csv` or `.json`
- **Clean output**: colored live table and optional step-by-step progress with spinners
- **Safe to stop**: press `Ctrl+C` and the results found so far are kept

## Installation

You need a recent stable [Rust toolchain](https://rustup.rs).

```bash
git clone https://github.com/musafir-xhu/Tor-Site-Finder.git
cd Tor-Site-Finder
cargo build --release
```

The program is then at `target/release/tor-link-finder`. To install it as a command you can run from anywhere:

```bash
cargo install --path .
```

## Usage

```bash
# search for a keyword and check every link found
tor-link-finder -S forum

# same search, show every step (token, links found, duplicates)
tor-link-finder -S forum -v

# save the results to a file instead of printing the table
tor-link-finder -S forum -o
```

| Flag | What it does |
|------|--------------|
| `-S` | Keyword to search for |
| `-o` | Save results to a file instead of printing the table (`tor_sites.txt` by default) |
| `-v`, `--verbose` | Show each step while the tool works |
| `-h`, `--help` | Show all options |
| `-V`, `--version` | Show the version |

Run `tor-link-finder --help` to see every option, including the file format and status-code filters.

## Example output

```text
╭──────┬─────────┬──────────────────────────────────┬─────────────────────────┬──────────────────────┬────────╮
│ No   │ Keyword │ Link                             │ Last Active             │ Time Ago             │ Status │
├──────┼─────────┼──────────────────────────────────┼─────────────────────────┼──────────────────────┼────────┤
│ 1    │ forum   │ http://a1b2c3d4e5f6g7h8i9j0k1..  │ Nov. 1, 2026, 6:24 a.m. │ 2 days, 17 hours     │ 200    │
│ 2    │ forum   │ http://p6q7r8s9t0u1v2w3x4y5z6..  │ Nov. 1, 2026, 1:57 a.m. │ 2 days, 22 hours     │ 200    │
│ 3    │ forum   │ http://e1f2g3h4i5j6k7l8m9n0o1..  │ Nov. 3, 2026, 8:30 a.m. │ 15 hours, 29 minutes │ 200    │
╰──────┴─────────┴──────────────────────────────────┴─────────────────────────┴──────────────────────┴────────╯
```

| Column | Meaning |
|--------|---------|
| **No** | Result number |
| **Keyword** | The keyword that found the link |
| **Link** | The onion address that was checked |
| **Last Active** | When Ahmia last saw the site online |
| **Time Ago** | How long ago that was |
| **Status** | HTTP status code the site answered with |

> The links above are placeholders, not real sites.

## Screenshots

<!-- Add screenshots here. Put the images in the assets/ folder and use, for example:
<p align="center">
  <img src="assets/screenshot.png" alt="Tor Link Finder in the terminal" width="90%">
</p>
-->

## How it works

```text
   keyword
      │
      ▼
 ┌───────────┐     ┌────────────────┐     ┌───────────────────┐     ┌──────────────┐
 │  Ahmia    │ ──▶ │ extract links  │ ──▶ │ check every link  │ ──▶ │ table / file │
 │  search   │     │ + remove dupes │     │ over Tor (async)  │     │ txt csv json │
 └───────────┘     └────────────────┘     └───────────────────┘     └──────────────┘
```

1. The tool searches Ahmia for your keyword and collects the onion links in the results.
2. Duplicate links are removed.
3. Every link is requested over Tor, with up to 50 requests running at the same time.
4. Each answer is printed live in the table, or saved to a file when you use `-o`.

## Good to know

- **The first run is slower.** Tor needs a short time to start up and build its circuits, so the first results can take a while to appear.
- **Onion sites are slow.** Each request has a timeout. A site that doesn't answer in time is skipped, and that does not always mean the site is down.
- **A status code is not a safety check.** `200` only means the site answered.

## Disclaimer

This tool is for learning, research and checking that services are reachable. It only lists links that a public search engine already shows, and it only reads the HTTP status of each site. Do not use it to access illegal content. You are responsible for how you use it and for following the laws where you live.

## License

Released under the [MIT License](LICENSE).
