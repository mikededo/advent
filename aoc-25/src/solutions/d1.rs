use utils::read_lines;

pub fn solve_a() {
    let (_, res) = read_lines("d1.txt", 25)
        .iter()
        .fold((50, 0), |(pos, count), l| {
            let (dir, amount) = l.split_at(1);
            let val: i32 = match dir {
                "L" => -amount.parse().unwrap_or(0),
                "R" => amount.parse().unwrap_or(0),
                _ => 0,
            };

            let new_pos = (pos + val) % 100;
            (new_pos, count + if pos == 0 { 1 } else { 0 })
        });

    println!("{res}")
}
