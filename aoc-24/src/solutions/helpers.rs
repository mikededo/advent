use std::{env, fs::read_to_string};

pub fn read_lines(path: &str) -> Vec<String> {
    let path = format!(
        "{root}/aoc-24/src/data/{path}",
        root = env::current_dir().unwrap().display()
    );
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
