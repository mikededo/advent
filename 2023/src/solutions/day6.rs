use super::helpers::read_lines;

pub fn solve_a() {
    let mut games: Vec<(usize, usize)> = Vec::new();
    for (i, l) in read_lines("day6.txt").iter().enumerate() {
        if let Some((_, v)) = l.split_once(':') {
            v.split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(j, val)| {
                    if i == 0 {
                        games.push((val, 0));
                    } else {
                        games[j].1 = val;
                    }
                });
        }
    }

    let res = games
        .iter()
        .map(|(dur, max)| {
            (0..=(dur / 2))
                .map(|v| (dur - v) * v)
                .filter(|v| v > max)
                .count()
                * 2
                - (dur % 2 == 0) as usize
        })
        .product::<usize>();
    println!("Total: {res}");
}

pub fn solve_b() {
    let values = read_lines("day6.txt")
        .iter()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .replace(' ', "")
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>();

    let dur = &values[0];
    let max = &values[1];
    println!(
        "Total: {}",
        (0..=(dur / 2))
            .map(|v| (dur - v) * v)
            .filter(|v| v > max)
            .count()
            * 2
            - (dur % 2 == 0) as usize
    );
}
