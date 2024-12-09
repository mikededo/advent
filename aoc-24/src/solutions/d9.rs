use utils::{read_chars_with, read_lines};

pub fn solve_a() {
    let bind = read_lines("d9.txt", 24);
    let res = bind.first().unwrap();
    let mut count = 0;
    // Build the sequence
    let mut seq: Vec<i32> = Vec::new();
    for (i, b) in res.bytes().enumerate() {
        let amount = (b - b'0') as usize;

        if i % 2 == 1 {
            for _ in 0..amount {
                seq.push(-1);
            }
        } else {
            for _ in 0..amount {
                seq.push(count);
            }
            count += 1;
        }
    }

    let mut last = seq.len() - 1;
    let mut i = 0;
    while i < seq.len() && i < last {
        if seq[i] == -1 {
            let mut c_last = seq[last];
            while c_last == -1 {
                last -= 1;
                c_last = seq[last];
            }
            seq[i] = c_last;
            seq[last] = -1;
        }
        i += 1;
    }

    let mut sum: i64 = 0;
    i = 0;
    while seq[i] != -1 {
        sum += seq[i] as i64 * i as i64;
        i += 1;
    }

    println!("{sum}");
}

pub fn solve_b() {
    let mut spaces: Vec<(usize, i32)> = Vec::new();
    let mut curr_index: i32 = 0;
    read_chars_with("d9.txt", 24, |c, (_, col)| {
        let amount = (c as u8 - b'0') as usize;
        if col % 2 == 1 {
            spaces.push((amount, -1));
        } else {
            spaces.push((amount, curr_index));
            curr_index += 1;
        }
    });

    let mut pair_index = 0;
    let pair_count = spaces.len();
    while pair_index < pair_count {
        let (amount, value) = spaces[pair_index];
        if value == -1 {
            let r_found =
                (pair_index + 1..pair_count).rfind(|&j| spaces[j].1 != -1 && spaces[j].0 <= amount);

            if let Some(j) = r_found {
                let found = spaces[j];

                if found.0 == amount {
                    spaces.swap(pair_index, j);
                } else {
                    let diff = (amount - found.0, -1);
                    spaces[pair_index] = found;
                    spaces[j] = (found.0, -1);
                    spaces.insert(pair_index + 1, diff);
                }
            }
        }

        pair_index += 1;
    }

    let mut position: i32 = 0;
    let res = spaces.iter().fold(0, |acc, &(amount, value)| {
        if value == -1 {
            position += amount as i32;
            return acc;
        }

        let sum: u64 = (0..amount)
            .map(|_| {
                let score = (value * position) as u64;
                position += 1;
                score
            })
            .sum();
        acc + sum
    });
    println!("{res}");
}
