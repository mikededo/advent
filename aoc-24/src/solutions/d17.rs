use itertools::Itertools;
use utils::read_lines;

#[derive(Debug)]
enum Command {
    Adv, // 0
    Bxl, // 1
    Bst, // 2
    Jnz, // 3
    Bxc, // 4
    Out, // 5
    Bdv, // 6
    Cdv, // 7
}
impl From<i64> for Command {
    fn from(c: i64) -> Self {
        match c {
            0 => Command::Adv,
            1 => Command::Bxl,
            2 => Command::Bst,
            3 => Command::Jnz,
            4 => Command::Bxc,
            5 => Command::Out,
            6 => Command::Bdv,
            7 => Command::Cdv,
            _ => panic!("Invalid command"),
        }
    }
}

#[derive(Debug)]
struct Program {
    ra: i64,
    rb: i64,
    rc: i64,
    pointer: usize,
    instructions: Vec<i64>,
    output: Vec<i64>,
}
impl Program {
    fn new(ra: i64, rb: i64, rc: i64, instructions: Vec<i64>) -> Self {
        Self {
            ra,
            rb,
            rc,
            pointer: 0,
            instructions,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.pointer < self.instructions.len() {
            let (opcode, operand) = (
                self.instructions[self.pointer],
                self.instructions[self.pointer + 1],
            );

            match Command::from(opcode) {
                Command::Adv => {
                    let denom = 2u32.pow(self.operand_or_register(operand) as u32) as f64;
                    let res = (self.ra as f64 / denom).trunc() as i64;

                    self.ra = res;
                }
                Command::Bxl => self.rb ^= operand,
                Command::Bst => self.rb = self.operand_or_register(operand) % 8,
                Command::Jnz => {
                    if self.ra != 0 {
                        self.pointer = self.operand_or_register(operand) as usize;
                        continue;
                    }
                }
                Command::Bxc => self.rb ^= self.rc,
                Command::Out => {
                    let res = self.operand_or_register(operand) % 8;
                    self.output.push(res);
                }
                Command::Bdv => {
                    let denom = 2u32.pow(self.operand_or_register(operand) as u32) as f64;
                    let res = (self.ra as f64 / denom).trunc() as i64;

                    self.rb = res;
                }
                Command::Cdv => {
                    let denom = 2u32.pow(self.operand_or_register(operand) as u32) as f64;
                    let res = (self.ra as f64 / denom).trunc() as i64;

                    self.rc = res;
                }
            }

            self.pointer += 2;
        }
    }

    fn operand_or_register(&self, operand: i64) -> i64 {
        match operand {
            0..4 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!(),
        }
    }
}

pub fn solve_a() {
    let ((ra, rb, rc), ins) = read_lines("d17.txt", 24).iter().enumerate().fold(
        ((0, 0, 0), Vec::new()),
        |agg, (i, line)| {
            if i < 3 {
                let s = line.split(": ").last().unwrap().parse().unwrap();
                return match i % 3 {
                    0 => ((s, agg.0 .1, agg.0 .2), agg.1),
                    1 => ((agg.0 .0, s, agg.0 .2), agg.1),
                    2 => ((agg.0 .0, agg.0 .1, s), agg.1),
                    _ => agg,
                };
            } else if i == 4 {
                let ins = line
                    .split(": ")
                    .last()
                    .unwrap()
                    .split(",")
                    .map(|c| c.parse().unwrap())
                    .collect::<Vec<i64>>();
                return (agg.0, ins);
            }

            agg
        },
    );

    let mut program = Program::new(ra, rb, rc, ins);
    program.run();

    println!(
        "{:?}",
        program
            .output
            .iter()
            .format_with(",", |x, f| f(&x.to_string()))
    );
}
