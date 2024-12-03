use std::fs::write;
use std::path::Path;

use chrono::{Datelike, Utc};
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long)]
    cookie: String,

    #[arg(short, long)]
    date: u32,

    #[arg(short, long)]
    year: Option<i32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let args = Args::parse();

    let year = args.year.unwrap_or_else(|| Utc::now().year());
    let url = format!(
        "https://adventofcode.com/{year}/day/{date}/input",
        date = args.date,
        year = year,
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&format!("session={}", args.cookie))?,
    );

    let response = client.get(url).headers(headers).send()?;
    if response.status().is_success() {
        // Write into the file
        let path = format!(
            "./aoc-{year}/src/data/d{date}.txt",
            year = year % 100,
            date = args.date
        );
        let file_path = Path::new(&path);
        write(file_path, response.text()?)?;
        println!("Wrote to {}", file_path.display());
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())
}
