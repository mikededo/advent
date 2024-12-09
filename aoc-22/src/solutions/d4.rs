use utils::read_lines;

fn check_if_overlap<F>(check: F) -> usize
where
    F: Fn((i32, i32), (i32, i32)) -> bool,
{
    let res = read_lines("d4.txt", 22)
        .iter()
        .filter(|line| {
            let ranges = line
                .split(',')
                .map(|r| {
                    let mut range = r.split('-').map(|n| n.parse::<i32>().unwrap());
                    (range.next().unwrap(), range.next().unwrap())
                })
                .collect::<Vec<(i32, i32)>>();

            let r1 = ranges[0];
            let r2 = ranges[1];

            // Full overlap
            check((r1.0, r1.1), (r2.0, r2.1))
        })
        .count();
    res
}

pub fn solve_a() {
    let res =
        check_if_overlap(|r1, r2| r1.0 <= r2.0 && r1.1 >= r2.1 || r2.0 <= r1.0 && r2.1 >= r1.1);

    println!("{res:?}");
}

pub fn solve_b() {
    let res = check_if_overlap(|r1, r2| {
        r1.0 >= r2.0 && r1.0 <= r2.1
            || r1.1 >= r2.0 && r1.1 <= r2.1
            || r2.0 >= r1.0 && r2.0 <= r1.1
            || r2.1 >= r1.0 && r2.1 <= r1.1
    });

    println!("{res:?}");
}
