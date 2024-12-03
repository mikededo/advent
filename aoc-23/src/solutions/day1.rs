use crate::solutions::helpers::read_lines;

fn sum_first_last(mut digits: impl Iterator<Item = u32>) -> u32 {
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);
    first * 10 + last
}

fn find_digits(s: &str) -> u32 {
    let digits = s
        .chars()
        .filter(|c| c.is_ascii_digit())
        .flat_map(|c| c.to_digit(10));
    sum_first_last(digits)
}

pub fn solve_a() {
    println!(
        "{:?}",
        read_lines("day1.txt")
            .into_iter()
            .map(|s| find_digits(&s))
            .sum::<u32>()
    );
}

pub fn solve_b() {
    let words: Vec<(u32, &str)> = (1..=9)
        .zip([
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ])
        .collect();
    let result = read_lines("day1.txt")
        .into_iter()
        .map(|s| {
            let mut matches: Vec<u32> = vec![];
            s.chars().fold("".to_owned(), |mut acc, c| {
                acc.push(c);
                if let Some((num, _)) = words.iter().find(|(_, word)| acc.ends_with(word)) {
                    matches.push(*num);
                } else if let Some(num) = c.to_digit(10) {
                    matches.push(num)
                }
                acc
            });
            matches
        })
        .map(|matches| sum_first_last(matches.into_iter()))
        .sum::<u32>();
    println!("{:?}", result)
}
