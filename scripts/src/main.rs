use std::fs::write;
use std::path::Path;

use chrono::Datelike;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

type Res = Result<(), Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    New {
        #[arg(required = true)]
        number: u32,
        #[arg(short, long, value_parser = clap::value_parser!(u32))]
        year: Option<u32>,
    },

    Data {
        #[arg(long, required = true)]
        cookie: String,
        #[arg(long, value_parser = clap::value_parser!(u32))]
        day: u32,
        #[arg(long, value_parser = clap::value_parser!(u32))]
        year: Option<u32>,
    },
}

fn get_problem_data(cookie: &str, day: u32, year: u32) -> Res {
    let client = Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input",);
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&format!("session={cookie}"))?);

    let response = client.get(url).headers(headers).send()?;
    if response.status().is_success() {
        // Write into the file
        let path = format!("./aoc-{year}/src/data/d{day}.txt", year = year % 100,);
        let file_path = Path::new(&path);
        write(file_path, response.text()?)?;
        println!("Wrote to {}", file_path.display());
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())
}

fn new_problem(number: u32, year: u32) -> Res {
    // Write into the file
    let path = format!("./aoc-{year}/src/solutions/d{number}.rs", year = year % 100,);
    let file_path = Path::new(&path);
    if file_path.exists() {
        return Err(format!("File already exists: {}", file_path.display()).into());
    }

    let content = r#"
use super::helpers::read_lines;

pub fn solve_a() {
    let res: i32 = read_lines("d{number}.txt")
        .iter();
        # TODO: Solve part
        .sum()

    println!("{res}");
}
"#;
    if let Some(value) = write_to_file(file_path, content) {
        return value;
    }

    println!("Wrote to {}", file_path.display());
    Ok(())
}

fn write_to_file(file_path: &Path, content: &str) -> Option<Res> {
    let write_res = write(file_path, content);
    if let Err(write_err) = write_res {
        return Some(Err(Box::new(write_err)));
    }
    None
}

fn year_or_current(y: Option<u32>) -> u32 {
    match y {
        Some(y) => y,
        None => chrono::Local::now().year() as u32,
    }
}

fn main() -> Res {
    let cli = Cli::parse();

    // Attempt to match and pass specific command variant
    match &cli.command {
        Commands::Data { cookie, day, year } => {
            get_problem_data(cookie, *day, year_or_current(*year))
        }

        Commands::New { number, year } => new_problem(*number, year_or_current(*year)),
    }
}
