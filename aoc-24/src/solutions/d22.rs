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
