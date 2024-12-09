use utils::read_lines;

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
