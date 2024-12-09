use std::collections::HashSet;

use utils::read_lines;

fn prio(value: u8) -> usize {
    match value {
        (b'a'..=b'z') => 1 + (value - b'a') as usize,
        (b'A'..=b'Z') => 27 + (value - b'A') as usize,
        _ => unreachable!(),
    }
}

pub fn solve_a() {
    let res = read_lines("d3.txt", 22).iter().fold(0, |acc, line| {
        let (p1, p2) = line.split_at(line.len() / 2);
        let set: HashSet<char> = HashSet::from_iter(p1.chars());
        let found = p2
            .chars()
            .find(|c| set.contains(c))
            .unwrap_or_else(|| unreachable!());

        acc + prio(found as u8)
    });

    println!("{res}");
}

pub fn solve_b() {
    let mut i = 0;
    let mut set_one: HashSet<char> = HashSet::new();
    let mut set_two: HashSet<char> = HashSet::new();

    let res = read_lines("d3.txt", 22).iter().fold(0, |acc, line| {
        i += 1;
        if i == 1 {
            set_one.extend(line.chars());
        } else if i == 2 {
            set_two.extend(line.chars());
        } else {
            let found = line
                .chars()
                .find(|c| set_one.contains(c) && set_two.contains(c))
                .unwrap_or_else(|| unreachable!());

            i = 0;
            set_one.clear();
            set_two.clear();

            return acc + prio(found as u8);
        }

        acc
    });

    println!("{res}");
}
