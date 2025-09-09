use std::collections::HashSet;

use utils::read_lines;

pub fn solve_a() {
    let sol = read_lines("d6.txt", 22)
        .iter()
        .map(|line| {
            line.as_bytes()
                .windows(4)
                .position(|win| HashSet::<&u8>::from_iter(win).len() == 4)
                .map(|p| p + 4)
                .unwrap_or(0)
        })
        .sum::<usize>();

    println!("{}", sol);
}
