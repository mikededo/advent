use super::helpers::read_lines;
use regex::Regex;

pub fn solve_a() {
    let mul_re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();

    let res: i32 = read_lines("d3.txt")
        .iter()
        .map(|line| {
            mul_re
                .find_iter(line)
                .map(|f| parse_nums(f.as_str()))
                .fold(0, |acc, (x, y)| acc + x * y)
        })
        .sum();

    println!("{res}");
}

pub fn solve_b() {
    let re = Regex::new(r"(do\(\))|(don\'t\(\))|(mul\(\d+,\d+\))").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don\'t\(\)").unwrap();

    let mut active = true;
    let res: i32 = read_lines("d3.txt")
        .iter()
        .map(|line| {
            re.find_iter(line)
                .map(|f| {
                    let f_val = f.as_str();
                    if do_re.is_match(f_val) {
                        active = true;
                        return (0, 0);
                    }
                    if dont_re.is_match(f_val) {
                        active = false;
                        return (0, 0);
                    }
                    if !active {
                        return (0, 0);
                    }

                    parse_nums(f.as_str())
                })
                .fold(0, |acc, (x, y)| acc + x * y)
        })
        .sum();

    println!("{res}");
}

fn parse_nums(m: &str) -> (i32, i32) {
    let num_re = Regex::new(r"\d+").unwrap();
    let found = num_re
        .find_iter(m)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    (
        found[0].parse::<i32>().unwrap(),
        found[1].parse::<i32>().unwrap(),
    )
}
