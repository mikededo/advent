use std::{num::ParseIntError, str::FromStr};

use utils::read_lines;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Move form
        // move x from y to z
        let parts = s.split(' ').collect::<Vec<&str>>();

        Ok(Self {
            count: parts[1].parse::<usize>()?,
            from: parts[3].parse::<usize>()?,
            to: parts[5].parse::<usize>()?,
        })
    }
}

struct Solver {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl Solver {
    pub fn solve_iterative(mut self) -> String {
        self.moves.iter().for_each(|m| {
            for _ in 0..m.count {
                let from = m.from - 1;
                let to = m.to - 1;

                if let Some(c) = self.stacks[from].pop() {
                    self.stacks[to].push(c);
                }
            }
        });

        self.get_result()
    }

    pub fn solve_chunks(mut self) -> String {
        for m in &self.moves {
            let from = m.from - 1;
            let to = m.to - 1;
            let from_len = self.stacks[from].len();

            let items = self.stacks[from].split_off(from_len.saturating_sub(m.count));
            self.stacks[to].extend(items);
        }

        self.get_result()
    }

    fn get_result(self) -> String {
        self.stacks.iter().fold(String::new(), |mut acc, stack| {
            if let Some(c) = stack.last() {
                acc.push(*c);
            }
            acc
        })
    }
}

fn parse_stacks(stack_lines: &[String]) -> Vec<Vec<char>> {
    let num_stacks = (stack_lines[0].len() + 1) / 4;
    let mut stacks = vec![vec![]; num_stacks];

    for line in stack_lines {
        for (i, chunk) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            if chunk[1] != ' ' {
                stacks[i].insert(0, chunk[1]);
            }
        }
    }

    stacks
}

pub fn solve_a() {
    let lines = read_lines("d5.txt", 22);
    let split_idx = lines.iter().position(|line| line.is_empty()).unwrap();

    println!(
        "{}",
        Solver {
            stacks: parse_stacks(&lines[..split_idx - 1]),
            moves: lines[split_idx + 1..lines.len()]
                .iter()
                .map(|line| Move::from_str(line).unwrap())
                .collect(),
        }
        .solve_iterative()
    );
}

pub fn solve_b() {
    let lines = read_lines("d5.txt", 22);
    let split_idx = lines.iter().position(|line| line.is_empty()).unwrap();

    println!(
        "{}",
        Solver {
            stacks: parse_stacks(&lines[..split_idx - 1]),
            moves: lines[split_idx + 1..lines.len()]
                .iter()
                .map(|line| Move::from_str(line).unwrap())
                .collect(),
        }
        .solve_chunks()
    );
}
