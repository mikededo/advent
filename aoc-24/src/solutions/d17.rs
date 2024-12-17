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
impl From<u64> for Command {
    fn from(c: u64) -> Self {
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
    ra: u64,
    rb: u64,
    rc: u64,
    pointer: usize,
    instructions: Vec<u64>,
}
impl Program {
    fn new(ra: u64, rb: u64, rc: u64, instructions: Vec<u64>) -> Self {
        Self {
            ra,
            rb,
            rc,
            pointer: 0,
            instructions,
        }
    }

    fn run(&mut self) -> String {
        self.run_vec().iter().map(|x| x.to_string()).collect()
    }

    fn run_vec(&mut self) -> Vec<u64> {
        let mut res = Vec::new();

        while self.pointer < self.instructions.len() {
            let (opcode, operand) = (
                self.instructions[self.pointer],
                self.instructions[self.pointer + 1],
            );

            match Command::from(opcode) {
                Command::Adv => {
                    let denom = 2u64.pow(self.operand_or_register(operand) as u32) as f64;
                    let result = (self.ra as f64 / denom).trunc() as u64;
                    self.ra = result;
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
                    res.push(self.operand_or_register(operand) % 8);
                }
                Command::Bdv | Command::Cdv => {
                    let denom = 2u64.pow(self.operand_or_register(operand) as u32) as f64;
                    let result = (self.ra as f64 / denom).trunc() as u64;

                    if matches!(Command::from(opcode), Command::Bdv) {
                        self.rb = result;
                    } else {
                        self.rc = result;
                    }
                }
            }

            self.pointer += 2;
        }

        res
    }

    fn operand_or_register(&self, operand: u64) -> u64 {
        match operand {
            0..4 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!(),
        }
    }

    fn reset(&mut self, ra: u64, rb: u64, rc: u64) {
        self.ra = ra;
        self.rb = rb;
        self.rc = rc;
        self.pointer = 0;
    }
}

fn parse_reg(s: &str) -> u64 {
    s.split(": ").last().unwrap().parse().unwrap()
}

pub fn solve_a() {
    let lines = read_lines("d17.txt", 24);
    let mut program = Program::new(
        parse_reg(&lines[0]),
        parse_reg(&lines[1]),
        parse_reg(&lines[2]),
        lines[4]
            .split(": ")
            .last()
            .unwrap()
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect(),
    );

    println!("{}", program.run());
}

pub fn solve_b() {
    let lines = read_lines("d17.txt", 24);
    let expected: Vec<u64> = lines[4]
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();
    let mut program = Program::new(
        parse_reg(&lines[0]),
        parse_reg(&lines[1]),
        parse_reg(&lines[2]),
        expected.clone(),
    );

    // Powers of 8
    let pow_8: Vec<u64> = (0..expected.len()).map(|i| 8u64.pow(i as u32)).collect();

    let mut a: u64 = 0;
    let mut i = expected.len() - 1;
    loop {
        program.reset(a, 0, 0);

        let mut output = program.run_vec();
        while output.len() < expected.len() {
            output.push(10);
        }

        let correct = output[i..]
            .iter()
            .enumerate()
            .all(|(j, x)| *x == expected[i..][j]);

        if correct {
            // All numbers from i..output.len() match, so we can check for the previous digit or
            // finish
            if i == 0 {
                break;
            } else {
                i -= 1
            }
        } else {
            // A digit changes every 8^n steps, so whenever we find an a register that matches from
            // i..output.len(), we add 8^i to calculate for the next digit
            a += pow_8[i];
        }
    }

    println!("{a}");
}
