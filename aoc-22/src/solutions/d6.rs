use std::collections::HashSet;

use utils::read_lines;

fn solve(n: usize) -> usize {
    read_lines("d6.txt", 22)
        .iter()
        .map(|line| {
            line.as_bytes()
                .windows(n)
                .position(|win| HashSet::<&u8>::from_iter(win).len() == n)
                .map(|p| p + n)
                .unwrap_or(0)
        })
        .sum()
}

pub fn solve_a() {
    println!("{}", solve(4));
}

pub fn solve_b() {
    println!("{}", solve(14));
}
