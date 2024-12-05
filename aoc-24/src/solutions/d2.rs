use utils::read_lines;

fn is_valid_range(x: i32, y: i32) -> bool {
    (1..=3).contains(&(x - y).abs())
}

fn check<I>(v: I, carry_errors: i32) -> bool
where
    I: Iterator<Item = i32>,
{
    let mut prev = i32::MIN;
    // 0 not init, 1 increasing, -1 decreasing
    let mut increasing: i32 = 0;
    let mut error_count = carry_errors;

    v.enumerate().all(|(i, x)| {
        if prev == i32::MIN {
            prev = x;
            return true;
        }

        if increasing == 0 || (i == 2 && error_count == 1) {
            increasing = if x > prev { 1 } else { -1 };
        }

        if (x > prev && increasing == -1) || (x < prev && increasing == 1) {
            if error_count == 0 {
                error_count = 1;
                return true;
            }

            return false;
        }

        let valid = is_valid_range(x, prev);
        if error_count == 0 && !valid {
            error_count = 1;
            return true;
        }
        prev = x;
        valid
    })
}

pub fn solve_a() {
    let count: i32 = read_lines("d2.txt", 24)
        .iter()
        .map(|line| check(line.split(" ").map(|x| x.parse::<i32>().unwrap()), 1) as i32)
        .sum();

    println!("{count}");
}

pub fn solve_b() {
    let count: i32 = read_lines("d2.txt", 24)
        .iter()
        .map(|line| {
            let entries: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            // Check all keeping first || Check after removing first - no errors allowed
            (check(entries.iter().cloned(), 0) || check(entries[1..].iter().cloned(), 1)) as i32
        })
        .sum();

    println!("{count}");
}
