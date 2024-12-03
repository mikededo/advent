use std::fs::read_to_string;

pub fn read_lines(path: &str) -> Vec<String> {
    read_to_string(format!("./src/data/{path}"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

