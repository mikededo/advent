use super::helpers::read_lines;

fn is_valid_range(x: i32, y: i32) -> bool {
    (1..=3).contains(&i32::abs(x - y))
}

pub fn solve_a() {
    let count: i32 = read_lines("d2.txt")
        .iter()
        .map(|line| {
            let mut prev = i32::MIN;
            // 0 not init, 1 increasing, -1 decreasing
            let mut increasing: i32 = 0;

            line.split(" ").map(|x| x.parse::<i32>().unwrap()).all(|x| {
                if prev == i32::MIN {
                    prev = x;
                    return true;
                }

                if increasing == 0 {
                    increasing = if x > prev { 1 } else { -1 };
                }

                if (x > prev && increasing == -1) || (x < prev && increasing == 1) {
                    return false;
                }

                let valid = is_valid_range(x, prev);
                prev = x;
                valid
            }) as i32
        })
        .sum();

    println!("{count}");
}
