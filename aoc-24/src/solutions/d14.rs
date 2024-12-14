use regex::Regex;
use utils::read_lines;

const EX: i32 = 101;
const EY: i32 = 103;

// Intial position Xi => 3 | Movement Xm => 2
// For 5 iterations, we have:
// fx = Xi + Xm + Xm + Xm + Xm + Xm = Xi + 5Xm
//
// If we want to know where it ends in a specific square,
// we simply use the mod operator
fn find_end_pos(x: i32, y: i32, mx: i32, my: i32, count: i32) -> (i32, i32) {
    (
        (x + (mx * count)).rem_euclid(EX),
        (y + (my * count)).rem_euclid(EY),
    )
}

pub fn solve_a() {
    let n_re = Regex::new(r"((\-)?\d+)").unwrap();
    let mut q = [0; 4];
    read_lines("d14.txt", 24).iter().for_each(|l| {
        let values = n_re
            .find_iter(l)
            .map(|m| m.as_str().parse().unwrap())
            .collect::<Vec<i32>>();
        let (x, y) = find_end_pos(values[0], values[1], values[2], values[3], 100);

        if (0..EX / 2).contains(&x) && (0..EY / 2).contains(&y) {
            q[0] += 1;
        }
        if (1 + EX / 2..EX).contains(&x) && (0..EY / 2).contains(&y) {
            q[1] += 1;
        }
        if (0..EX / 2).contains(&x) && (1 + EY / 2..EY).contains(&y) {
            q[2] += 1;
        }
        if (1 + EX / 2..EX).contains(&x) && (1 + EY / 2..EY).contains(&y) {
            q[3] += 1;
        }
    });

    println!("{:?}", q[0] * q[1] * q[2] * q[3]);
}
