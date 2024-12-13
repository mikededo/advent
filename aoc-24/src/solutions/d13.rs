use core::fmt;
use num_traits::Float;
use std::{iter::Sum, str::FromStr};

use regex::Regex;
use utils::read_lines;

// To solve the problem use linear systems
//
// A0*m + B0*n = P0 => m = (P0 - B0*n) / A0
// A1*m + B1*n = P1 => m = (P1 - B1*n) / A1
//
// Equate => (P0 - B0*n) / A0 = (P1 - B1*n) / A1
//        => (P0 - B0*n) * A1 = (P1 - B1*n) * A0
//        => P0*A1 - B0*n*A1 = P1*A0 - B1*n*A0
//        => P0*A1 - P1*A0 = B0*n*A1 - B1*n*A0
//        => P0*A1 - P1*A0 = (B0*A1 - B1*A0)*n
//        => (P0*A1 - P1*A0) / (B0*A1 - B1*A0) = n
// Once found n, we substitute it in one of the previous equations
fn solve_equation<T: Float>(a: (T, T), b: (T, T), rprize: (T, T), extra: T) -> (T, T) {
    let prize = (extra + rprize.0, extra + rprize.1);

    let n = (prize.0 * a.1 - prize.1 * a.0) / (b.0 * a.1 - b.1 * a.0);
    let m = (prize.0 - b.0 * n) / a.0;
    (m, n)
}

fn parse_line<T: Float + FromStr>(line: &str) -> (T, T)
where
    <T as FromStr>::Err: fmt::Debug,
{
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<T> = re
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    (nums[0], nums[1])
}

fn solve<T: Float + Sum + FromStr>(extra: T) -> T
where
    <T as FromStr>::Err: fmt::Debug,
{
    let sum: T = read_lines("d13.txt", 24)
        .chunks(4)
        .filter_map(|c| {
            let (m, n) = solve_equation(
                parse_line(&c[0]),
                parse_line(&c[1]),
                parse_line(&c[2]),
                extra,
            );

            let res = m * T::from(3).unwrap() + n;
            if res.fract() == T::from(0).unwrap() {
                Some(res)
            } else {
                None
            }
        })
        .sum();
    sum
}

pub fn solve_a() {
    println!("{}", solve(0f32));
}

pub fn solve_b() {
    println!("{}", solve(10000000000000f64));
}
