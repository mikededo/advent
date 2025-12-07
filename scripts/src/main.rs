use std::{
    fs::{self, write, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use chrono::Datelike;
use clap::{Parser, Subcommand};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, COOKIE},
};

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
        #[arg(long, required = false, default_value = "")]
        cookie: String,
        #[arg(long, value_parser = clap::value_parser!(u32))]
        day: u32,
        #[arg(long, value_parser = clap::value_parser!(u32))]
        year: Option<u32>,
    },
}

fn read_env(key: &str) -> Option<String> {
    let file = fs::read_to_string(".env").ok()?;
    file.lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| line.split('=').nth(1))
        .map(String::from)
}

fn get_problem_data(cookie: &str, day: u32, year: u32) -> Res {
    let parsed_cookie = if cookie.is_empty() {
        &read_env("COOKIE").unwrap_or_default()
    } else {
        cookie
    };
    if parsed_cookie.is_empty() {
        return Err("Missing cookie variable".into());
    }

    let url = format!("https://adventofcode.com/{year}/day/{day}/input",);
    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&format!("session={parsed_cookie}"))?,
    );

    let response = Client::new().get(url).headers(headers).send()?;
    if response.status().is_success() {
        let path = format!("./aoc-{year}/src/data/d{day}.txt", year = year % 100);
        let file_path = Path::new(&path);
        write(file_path, response.text()?)?;
        println!("Wrote to {}", file_path.display());
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())
}

fn add_to_main(number: u32, year: u32) -> Res {
    let mod_path = format!("./aoc-{year}/src/solutions/mod.rs", year = year % 100,);
    let mod_line = format!("pub mod d{number};");
    let mod_file = Path::new(&mod_path);

    // Check if file exists
    if mod_file.exists() {
        let reader = BufReader::new(File::open(mod_file)?);
        if reader.lines().any(|line| line.unwrap().trim() == mod_line) {
            return Ok(());
        }

        // Append the line if does not exist
        let mut file = OpenOptions::new().append(true).open(mod_file)?;
        writeln!(file, "{mod_line}")?;
    } else {
        // Create the file if it does not exist
        let mut file = File::create(mod_file)?;
        writeln!(file, "pub mod d{number};")?;
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

    let content = r#"use utils::read_lines;

pub fn solve_a() {
    let res = read_lines("d{number}.txt");

    println!("{res}");
}
"#;
    if let Some(value) = write_to_file(file_path, content) {
        return value;
    }

    // Add to main.rs
    add_to_main(number, year)?;

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
