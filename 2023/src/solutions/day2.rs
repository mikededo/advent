use crate::solutions::helpers::read_lines;

const MAX_RED: u32 = 12;
const RED: &str = "red";
const MAX_GREEN: u32 = 13;
const GREEN: &str = "green";
const MAX_BLUE: u32 = 14;
const BLUE: &str = "blue";

pub fn solve_a() {
    let sum = read_lines("day2.txt")
        .into_iter()
        .map(|s| {
            s.split(": ")
                .skip(1)
                .flat_map(|s| s.split("; "))
                .map(|s| {
                    let (r, g, b) = s.split(", ").fold((true, true, true), |acc, s| {
                        let mut v = s.split(' ');
                        let num = v.next().unwrap().parse::<u32>().unwrap();
                        let color = v.last().unwrap();
                        match color {
                            RED => (num <= MAX_RED, acc.1, acc.2),
                            GREEN => (acc.0, num <= MAX_GREEN, acc.2),
                            BLUE => (acc.0, acc.1, num <= MAX_BLUE),
                            _ => acc,
                        }
                    });
                    r && g && b
                })
                .all(|b| b)
        })
        .enumerate()
        .fold(0, |acc, (i, valid)| if valid { (i + 1) + acc } else { acc });
    println!("{sum}");
}

pub fn solve_b() {
    let sum = read_lines("day2.txt").into_iter().fold(0, |acc, s| {
        let mut min = [0, 0, 0];
        s.split(": ").last().unwrap().split("; ").for_each(|s| {
            s.split(", ").for_each(|s| {
                let mut v = s.split(' ');
                let num = v.next().unwrap().parse::<u32>().unwrap();
                let color = v.last().unwrap();
                match color {
                    RED => min[0] = min[0].max(num),
                    GREEN => min[1] = min[1].max(num),
                    BLUE => min[2] = min[2].max(num),
                    _ => (),
                };
            });
        });
        acc + (min[0] * min[1] * min[2])
    });

    println!("{:?}", sum);
}
