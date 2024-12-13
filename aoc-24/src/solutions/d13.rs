use regex::Regex;
use utils::read_lines;

// A0*m + B0*n = P0 => m = (P0 - B0*n) / A0
// A1*m + B1*n = P1 => m = (P1 - B1*n) / A1
//
// Equate => (P0 - B0*n) / A0 = (P1 - B1*n) / A1
//        => (P0 - B0*n) * A1 = (P1 - B1*n) * A0
//        => P0*A1 - B0*n*A1 = P1*A0 - B1*n*A0
//        => P0*A1 - P1*A0 = B0*n*A1 - B1*n*A0
//        => P0*A1 - P1*A0 = (B0*A1 - B1*A0)*n
//        => (P0*A1 - P1*A0) / (B0*A1 - B1*A0) = n
// Once found n, we substitute it in one of the previous equations
fn solve(a: (f32, f32), b: (f32, f32), prize: (f32, f32)) -> (f32, f32) {
    let n = (prize.0 * a.1 - prize.1 * a.0) / (b.0 * a.1 - b.1 * a.0);
    let m = (prize.0 - b.0 * n) / a.0;
    (m, n)
}

fn parse_line(line: &str) -> (f32, f32) {
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<f32> = re
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

pub fn solve_a() {
    let res = read_lines("d13.txt", 24)
        .chunks(4)
        .filter_map(|c| {
            let (m, n) = solve(parse_line(&c[0]), parse_line(&c[1]), parse_line(&c[2]));

            let res = m * 3f32 + n;
            if res.fract() == 0f32 {
                Some(res)
            } else {
                None
            }
        })
        .sum::<f32>();

    println!("{res}");
}
