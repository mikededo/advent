use utils::read_lines;

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let values: Vec<&str> = line.split(": ").collect();
    (
        values[0].parse::<usize>().unwrap(),
        values[1]
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
    )
}

fn combine(want: usize, got: usize, received: &[usize]) -> bool {
    if received.is_empty() {
        return want == got;
    }

    let curr = received[0];
    let rest = &received[1..];
    combine(want, got + curr, rest) || combine(want, got * curr, rest)
}

pub fn solve_a() {
    let res = read_lines("d7.txt", 24).iter().fold(0, |acc, line| {
        let (expected, values) = parse_line(line);
        if combine(expected, 0, &values) {
            acc + expected
        } else {
            acc
        }
    });

    println!("{res}");
}

fn join_numbers(a: usize, b: usize) -> usize {
    let mut joined = a;
    let mut b_copy = b;

    while b_copy > 0 {
        joined *= 10;
        joined += b_copy % 10;
        b_copy /= 10;
    }

    joined
}

fn combine_b(want: usize, got: usize, received: &[usize]) -> bool {
    if received.is_empty() {
        return want == got;
    }

    let curr = received[0];
    let rest = &received[1..];
    combine_b(want, got + curr, rest)
        || combine_b(want, got * curr, rest)
        || combine_b(want, join_numbers(got, curr), rest)
}

pub fn solve_b() {
    let res = read_lines("d7.txt", 24).iter().fold(0, |acc, line| {
        let (expected, values) = parse_line(line);
        if combine_b(expected, 0, &values) {
            acc + expected
        } else {
            acc
        }
    });

    println!("{res}");
}
