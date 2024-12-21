use std::collections::HashMap;

use utils::read_lines;

// Key pad         Movement pad
// +---+---+---+       +---+---+
// | 7 | 8 | 9 |       | ^ | A |
// +---+---+---+   +---+---+---+
// | 4 | 5 | 6 |   | < | v | > |
// +---+---+---+   +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

fn keypad(k: char) -> (isize, isize) {
    match k {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!("Invalid keypad key: {k}"),
    }
}

fn movpad(k: char) -> (isize, isize) {
    match k {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!("Invalid movpad key: {k}"),
    }
}

struct Solver {
    cache: HashMap<((isize, isize), usize, bool), usize>,
    start: (isize, isize),
}
impl Solver {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            start: keypad('A'),
        }
    }

    fn execute(&mut self, (x, y): (isize, isize), count: usize, horizontal: bool) -> usize {
        let (nx, ny) = (x.unsigned_abs(), y.unsigned_abs());

        // Check how many moves do we need to take into each direction
        let mut steps = vec![if x > 0 { '^' } else { 'v' }; nx];
        steps.extend(vec![if y > 0 { '<' } else { '>' }; ny]);

        if horizontal {
            steps.reverse();
        }

        steps.push('A');

        if count == 0 {
            steps.len()
        } else {
            let mut start = movpad('A');

            steps
                .into_iter()
                .map(|c| {
                    let key = movpad(c);
                    let curr = start;
                    let next = (curr.0 - key.0, curr.1 - key.1);
                    start = key;

                    if next.0 == 0 || next.1 == 0 || (key == (1, 0) && curr.0 == 0) {
                        self.hit_or_new((next, count - 1, false))
                    } else if curr == (1, 0) && key.0 == 0 {
                        self.hit_or_new((next, count - 1, true))
                    } else {
                        std::cmp::min(
                            self.hit_or_new((next, count - 1, false)),
                            self.hit_or_new((next, count - 1, true)),
                        )
                    }
                })
                .sum()
        }
    }

    fn evaluate_line(&mut self, line: &str, count: usize) -> usize {
        let lval: usize = line[0..3].parse().unwrap();
        let moves: usize = line
            .chars()
            .map(|c| {
                let key = keypad(c);
                let curr = self.start;
                let next = (curr.0 - key.0, curr.1 - key.1);
                self.start = key;

                if curr.0 == 3 && key.1 == 0 {
                    self.hit_or_new((next, count, false))
                } else if curr.1 == 0 && next.0 == 3 {
                    self.hit_or_new((next, count, true))
                } else {
                    std::cmp::min(
                        self.hit_or_new((next, count, true)),
                        self.hit_or_new((next, count, false)),
                    )
                }
            })
            .sum();

        lval * moves
    }

    fn hit_or_new(&mut self, args: ((isize, isize), usize, bool)) -> usize {
        match self.cache.get(&args) {
            Some(value) => *value,
            None => {
                let value = self.execute(args.0, args.1, args.2);
                self.cache.insert(args, value);
                value
            }
        }
    }
}

pub fn solve_a() {
    let mut solver = Solver::new();
    let res = read_lines("d21.txt", 24)
        .iter()
        .map(|l| solver.evaluate_line(l, 2))
        .sum::<usize>();

    println!("{res}");
}

pub fn solve_b() {
    let mut solver = Solver::new();
    let res = read_lines("d21.txt", 24)
        .iter()
        .map(|l| solver.evaluate_line(l, 25))
        .sum::<usize>();

    println!("{res}");
}
