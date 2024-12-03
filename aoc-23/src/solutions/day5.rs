use std::{
    io::{stdout, Write},
    usize::MAX,
};

use super::helpers::read_lines;

trait AlmanacIter {
    fn next(&self, val: usize) -> usize;
    fn prev(&self, val: usize) -> usize;
}

#[derive(Debug, Clone)]
struct Almanac {
    dest: usize,
    origin: usize,
    range: usize,
}

impl Almanac {
    fn from_line(line: &str) -> Almanac {
        let mut ite = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

        Almanac {
            dest: ite.next().unwrap(),
            origin: ite.next().unwrap(),
            range: ite.next().unwrap(),
        }
    }

    fn next(&self, val: usize) -> Option<usize> {
        if val >= self.origin && val <= self.origin + self.range {
            Some(self.dest + val - self.origin)
        } else {
            None
        }
    }

    fn prev(&self, val: usize) -> Option<usize> {
        if val >= self.dest && val <= self.dest + self.range {
            Some(self.origin + val - self.dest)
        } else {
            None
        }
    }
}

impl AlmanacIter for Vec<Almanac> {
    fn next(&self, val: usize) -> usize {
        self.iter()
            .find(|r| r.next(val).is_some())
            .and_then(|r| r.next(val))
            .unwrap_or(val)
    }

    fn prev(&self, val: usize) -> usize {
        self.iter()
            .find(|r| r.prev(val).is_some())
            .and_then(|r| r.prev(val))
            .unwrap_or(val)
    }
}

fn parse_input() -> (Vec<Vec<Almanac>>, String) {
    let mut seeds: String = String::new();
    let mut nl_count = MAX;
    let mut ranges: Vec<Vec<Almanac>> = vec![vec![]; 7];

    for s in read_lines("day5.txt").iter() {
        if s.is_empty() {
            nl_count += 1;
            continue;
        }

        if nl_count == MAX {
            nl_count = 0;
            seeds = String::from(s);
            continue;
        }

        if !s.contains(':') {
            ranges[nl_count - 1].push(Almanac::from_line(s));
        }
    }

    (ranges, seeds)
}

fn calc_min(seeds: Vec<usize>, ranges: Vec<Vec<Almanac>>) -> usize {
    seeds
        .iter()
        .map(|&seed| ranges.iter().fold(seed, |acc, rs| rs.next(acc)))
        .min()
        .unwrap()
}

pub fn solve_a() {
    let (ranges, seeds_line) = parse_input();
    let seeds = seeds_line
        .split(':')
        .skip(1)
        .flat_map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap_or_default())
        })
        .collect::<Vec<usize>>();

    println!("{}", calc_min(seeds, ranges));
}

pub fn solve_b() {
    let (ranges, seeds_line) = parse_input();
    let seeds = seeds_line
        .split(':')
        .skip(1)
        .flat_map(|s| s.split_whitespace().map(|s| s.parse::<usize>().unwrap()))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0]..=(chunk[0] + chunk[1])))
        .collect::<Vec<_>>();
    let rev_ranges = ranges.iter().rev().collect::<Vec<_>>();

    for l in 0usize.. {
        stdout()
            .write_all(format!("\rChecking location {}", l).as_bytes())
            .unwrap();
        stdout().flush().unwrap();

        let seed = rev_ranges.iter().fold(l, |acc, rs| rs.prev(acc));
        for seed_range in seeds.iter() {
            if seed_range.contains(&seed) {
                println!();
                println!("{seed}");
                return;
            }
        }
    }
}
