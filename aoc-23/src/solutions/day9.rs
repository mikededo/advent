fn parse_history(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|number| -> i32 {
            number
                .parse()
                .unwrap_or_else(|_| panic!("Illegal number: {:?}", number))
        })
        .collect()
}

fn calc_extrapolation(history: Vec<i32>) -> i32 {
    let mut diffs = history
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    let mut acc = Vec::new();

    while !diffs.iter().all(|diff| *diff == 0) {
        let next_diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        if diffs.is_empty() {
            break;
        }

        acc.push(diffs[diffs.len() - 1]);
        diffs = next_diffs;
    }

    history[history.len() - 1] + acc.iter().sum::<i32>()
}

pub fn solve_a() {
    println!(
        "{}",
        include_str!("../data/day9.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(parse_history)
            .map(calc_extrapolation)
            .sum::<i32>()
    );
}

pub fn solve_b() {
    println!(
        "{}",
        include_str!("../data/day9.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(parse_history)
            .map(|h| h.into_iter().rev().collect::<Vec<i32>>())
            .map(calc_extrapolation)
            .sum::<i32>()
    );
}
