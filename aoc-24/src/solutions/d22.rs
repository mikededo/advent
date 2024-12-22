use std::collections::HashMap;

use itertools::Itertools;
use utils::read_lines;

fn next(n: isize) -> isize {
    let mut res = n;
    res ^= res * 64;
    res %= 16777216;
    res ^= res / 32;
    res %= 16777216;
    res ^= res * 2048;
    res %= 16777216;

    res
}

pub fn solve_a() {
    let res = read_lines("d22.txt", 24)
        .iter()
        .map(|l| {
            let mut n = l.parse().unwrap();
            for _ in 0..2000 {
                n = next(n);
            }
            n
        })
        .sum::<isize>();

    println!("{res}");
}

pub fn solve_b() {
    let res = read_lines("d22.txt", 24)
        .iter()
        .map(|l| {
            let mut n = l.parse::<isize>().unwrap();
            let mut res = Vec::new();

            (0..2000).for_each(|_| {
                res.push(n % 10);
                n = next(n);
            });

            res.into_iter()
                .rev()
                .tuple_windows::<(_, _, _, _, _)>()
                .map(|(a, b, c, d, e)| ((d - e, c - d, b - c, a - b), a))
                .collect::<HashMap<_, _>>()
        })
        .fold(HashMap::new(), |mut acc, seq| {
            seq.into_iter()
                .for_each(|(k, v)| *acc.entry(k).or_insert(0) += v);
            acc
        })
        .into_values()
        .max()
        .unwrap();

    println!("{res:?}");
}
