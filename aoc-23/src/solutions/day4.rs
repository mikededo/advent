use std::collections::HashSet;

use super::helpers::read_lines;

pub fn solve_a() {
    println!(
        "{}",
        read_lines("day4.txt")
            .iter()
            .flat_map(|s| s.split(':').skip(1))
            .map(|s| s.split('|'))
            .map(|mut v| {
                let left: HashSet<&str> = v
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .collect::<HashSet<&str>>();
                v.next().unwrap().split_whitespace().fold(0, |acc, s| {
                    if left.contains(&s) {
                        acc + 1
                    } else {
                        acc
                    }
                })
            })
            .map(|c| if c != 0 { 2u32.pow(c - 1) } else { c })
            .sum::<u32>()
    );
}

pub fn solve_b() {
    let parsed: Vec<usize> = read_lines("day4.txt")
        .iter()
        .flat_map(|s| s.split(':').skip(1))
        .map(|s| s.split('|'))
        .map(|mut v| {
            let left: HashSet<&str> = v.next().unwrap().split_whitespace().collect();
            v.next()
                .unwrap()
                .split_whitespace()
                .fold(0, |acc, s| acc + left.contains(&s) as usize)
        })
        .collect();

    let mut instances = vec![1; parsed.len()];
    for i in 0..parsed.len() {
        let c = parsed[i];
        let k = instances[i];
        for _ in 0..k {
            for j in 0..c {
                instances[i + j + 1] += 1;
            }
        }
    }

    println!("{:?}", instances.iter().sum::<usize>());
}
