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
        acc + (combine(expected, 0, &values) as usize)
    });

    println!("{res}");
}
