// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..

use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::solutions::helpers::read_lines;

#[derive(Debug)]
struct Value {
    adj: Vec<usize>,
    value: u32,
}

fn calc_adj_pos(x: usize, y: usize, line_size: usize) -> Vec<usize> {
    let mut res = Vec::new();

    for p in x..y {
        if x > line_size {
            res.push(p - line_size);
        }
        res.push(p + line_size);
    }

    if x % line_size != 0 {
        if x > line_size {
            res.push(x - 1 - line_size);
        }
        res.push(x - 1);
        res.push(x - 1 + line_size);
    }
    if y % line_size != line_size - 1 {
        if y > line_size {
            res.push(y - line_size);
        }
        res.push(y);
        res.push(y + line_size);
    }

    res
}

pub fn solve_a() {
    let mut sym_set: HashSet<usize> = HashSet::new();
    let mut line = 0;
    let res: Vec<Value> = read_lines("day3.txt")
        .iter()
        .flat_map(|s| {
            let num_reg = Regex::new(r"[0-9]+").unwrap();
            let sym_reg = Regex::new(r"[^0-9.]").unwrap();
            let shift = line * s.len();
            line += 1;

            sym_reg.find_iter(s).for_each(|m| {
                sym_set.insert(m.start() + shift);
            });
            num_reg
                .find_iter(s)
                .map(|m| Value {
                    adj: calc_adj_pos(m.start() + shift, m.end() + shift, s.len()),
                    value: m.as_str().parse::<u32>().unwrap(),
                })
                .collect::<Vec<Value>>()
        })
        .collect();

    println!(
        "{}",
        res.iter()
            .filter(|m| m.adj.iter().any(|adj| sym_set.contains(adj)))
            .map(|m| m.value)
            .sum::<u32>()
    );
}

pub fn solve_b() {
    let mut sym_set: HashMap<usize, (i32, i32)> = HashMap::new();
    let mut line = 0;
    let res: Vec<Value> = read_lines("day3.txt")
        .iter()
        .flat_map(|s| {
            let num_reg = Regex::new(r"[0-9]+").unwrap();
            let sym_reg = Regex::new(r"[^0-9.]").unwrap();
            let shift = line * s.len();
            line += 1;

            sym_reg.find_iter(s).for_each(|m| {
                sym_set.insert(m.start() + shift, (-1, -1));
            });
            num_reg
                .find_iter(s)
                .map(|m| Value {
                    adj: calc_adj_pos(m.start() + shift, m.end() + shift, s.len()),
                    value: m.as_str().parse::<u32>().unwrap(),
                })
                .collect::<Vec<Value>>()
        })
        .collect();
    res.iter().for_each(|m| {
        for adj in m.adj.iter() {
            if let Some((left, right)) = sym_set.get(adj) {
                let val = m.value.try_into().unwrap();
                sym_set.insert(
                    *adj,
                    (
                        if *left == -1 { val } else { *left },
                        if *left != -1 { val } else { *right },
                    ),
                );
            }
        }
    });

    println!(
        "{}",
        sym_set
            .iter()
            .map(|(_, (left, right))| left.max(&0) * right.max(&0))
            .sum::<i32>()
    );
}
