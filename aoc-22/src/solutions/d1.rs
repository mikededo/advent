use std::cmp;

use utils::read_lines;

pub fn solve_a() {
    let (max, _) = read_lines("d1.txt", 22)
        .iter()
        .fold((0, 0), |(max, curr), line| {
            if line.is_empty() {
                return (cmp::max(max, curr), 0);
            }

            (max, curr + line.parse::<u32>().unwrap_or(0))
        });

    println!("{max}");
}

pub fn solve_b() {
    let (max, _) = read_lines("d1.txt", 22)
        .iter()
        .fold(([0; 3], 0), |(max, curr), line| {
            if line.is_empty() {
                if max[0] < curr {
                    return ([curr, max[0], max[1]], 0);
                } else if max[1] < curr {
                    return ([max[0], curr, max[1]], 0);
                } else {
                    return ([max[0], max[1], cmp::max(max[2], curr)], 0);
                }
            }

            (max, curr + line.parse::<u32>().unwrap_or(0))
        });

    println!("{}", max.iter().sum::<u32>());
}
