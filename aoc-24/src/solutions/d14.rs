use std::{fs::File, io::Write};

use regex::Regex;
use utils::read_lines;

const EX: isize = 101;
const EY: isize = 103;

struct Robot {
    x: isize,
    y: isize,
    mx: isize,
    my: isize,
}
impl Robot {
    fn new(values: Vec<isize>) -> Self {
        Self {
            x: values[0],
            y: values[1],
            mx: values[2],
            my: values[3],
        }
    }

    // Intial position Xi => 3 | Movement Xm => 2
    // For 5 iterations, we have:
    // fx = Xi + Xm + Xm + Xm + Xm + Xm = Xi + 5Xm
    //
    // If we want to know where it ends in a specific square,
    // we simply use the mod operator
    fn step(&mut self, count: isize) {
        self.x = (self.x + (self.mx * count)).rem_euclid(EX);
        self.y = (self.y + (self.my * count)).rem_euclid(EY);
    }

    fn quadrant(&self) -> usize {
        if (0..EX / 2).contains(&self.x) && (0..EY / 2).contains(&self.y) {
            return 0;
        }
        if (1 + EX / 2..EX).contains(&self.x) && (0..EY / 2).contains(&self.y) {
            return 1;
        }
        if (0..EX / 2).contains(&self.x) && (1 + EY / 2..EY).contains(&self.y) {
            return 2;
        }
        if (1 + EX / 2..EX).contains(&self.x) && (1 + EY / 2..EY).contains(&self.y) {
            return 3;
        }
        4
    }
}

pub fn solve_a() {
    let n_re = Regex::new(r"((\-)?\d+)").unwrap();
    let mut q = [0; 5];
    read_lines("d14.txt", 24).iter().for_each(|l| {
        let values = n_re
            .find_iter(l)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let mut r = Robot::new(values);

        r.step(100);
        q[r.quadrant()] += 1;
    });

    println!("{:?}", q[0] * q[1] * q[2] * q[3]);
}

pub fn solve_b() {
    let n_re = Regex::new(r"((\-)?\d+)").unwrap();
    let mut robots = read_lines("d14.txt", 24)
        .iter()
        .map(|l| {
            let values = n_re
                .find_iter(l)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            Robot::new(values)
        })
        .collect::<Vec<_>>();

    let mut f = File::options()
        .create(true)
        .write(true)
        .truncate(false)
        .open("./14.txt")
        .unwrap();
    let mut c = 0;
    while c < 10000 {
        robots.iter_mut().for_each(|r| r.step(1));
        let _ = f.write_all(format!("{c}\n").as_bytes());

        for i in 0..EX {
            for j in 0..EY {
                let _ = match robots.iter().find(|r| r.x == i && r.y == j) {
                    Some(_) => f.write_all(b"X"),
                    None => f.write_all(b"."),
                };
            }
            let _ = f.write_all(b"\n");
        }
        c += 1;
    }
}
