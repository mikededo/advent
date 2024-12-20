use std::{env, fs::read_to_string};

pub fn read_lines(path: &str, year: u32) -> Vec<String> {
    let path = format!(
        "{root}/aoc-{year}/src/data/{path}",
        root = env::current_dir().unwrap().display()
    );
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_chars(path: &str, year: u32) -> Vec<Vec<String>> {
    let path = format!(
        "{root}/aoc-{year}/src/data/{path}",
        root = env::current_dir().unwrap().display()
    );
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(String::from).collect())
        .collect()
}

pub fn read_chars_with<F>(path: &str, year: u32, mut callback: F) -> Vec<Vec<String>>
where
    F: FnMut(char, (usize, usize)),
{
    let path = format!(
        "{root}/aoc-{year}/src/data/{path}",
        root = env::current_dir().unwrap().display()
    );
    read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    callback(c, (row, col));
                    String::from(c)
                })
                .collect()
        })
        .collect()
}

pub fn read_bytes(path: &str, year: u32) -> Vec<Vec<u8>> {
    let path = format!(
        "{root}/aoc-{year}/src/data/{path}",
        root = env::current_dir().unwrap().display()
    );
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.bytes().collect())
        .collect()
}

pub fn read_bytes_with<F>(path: &str, year: u32, mut callback: F) -> Vec<Vec<u8>>
where
    F: FnMut(u8, (usize, usize)),
{
    let path = format!(
        "{root}/aoc-{year}/src/data/{path}",
        root = env::current_dir().unwrap().display()
    );
    read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, b)| {
                    callback(b, (row, col));
                    b
                })
                .collect()
        })
        .collect()
}
