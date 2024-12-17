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
}
impl Program {
    fn new(ra: i64, rb: i64, rc: i64, instructions: Vec<i64>) -> Self {
        Self {
            ra,
            rb,
            rc,
            pointer: 0,
            instructions,
        }
    }

    fn run(&mut self) -> String {
        let mut res = Vec::new();

        while self.pointer < self.instructions.len() {
            let (opcode, operand) = (
                self.instructions[self.pointer],
                self.instructions[self.pointer + 1],
            );

            match Command::from(opcode) {
                Command::Adv => {
                    let denom = 2u32.pow(self.operand_or_register(operand) as u32) as f64;
                    let result = (self.ra as f64 / denom).trunc() as i64;
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
                    res.push((self.operand_or_register(operand) % 8).to_string());
                }
                Command::Bdv | Command::Cdv => {
                    let denom = 2u32.pow(self.operand_or_register(operand) as u32) as f64;
                    let result = (self.ra as f64 / denom).trunc() as i64;

                    if matches!(Command::from(opcode), Command::Bdv) {
                        self.rb = result;
                    } else {
                        self.rc = result;
                    }
                }
            }

            self.pointer += 2;
        }

        res.join(",")
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

fn parse_reg(s: &str) -> i64 {
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
