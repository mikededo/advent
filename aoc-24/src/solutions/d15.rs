use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::thread;
use std::time::Duration;
use std::{
    collections::VecDeque,
    fmt::{Formatter, Result},
};
use std::{
    fmt::Display,
    io::{self, Write},
};

use utils::read_chars;

fn is_move_horizontal(s: &str) -> bool {
    match s {
        ">" | "<" => true,
        "v" | "^" => false,
        _ => unreachable!(),
    }
}

fn get_next_pos(s: &str, (x, y): (usize, usize)) -> (usize, usize) {
    match s {
        ">" => (x, y + 1),
        "<" => (x, y.wrapping_sub(1)),
        "v" => (x + 1, y),
        "^" => (x.wrapping_sub(1), y),
        _ => unreachable!(),
    }
}

fn get_prev_pos(s: &str, (x, y): (usize, usize)) -> (usize, usize) {
    match s {
        ">" => (x, y.wrapping_sub(1)),
        "<" => (x, y + 1),
        "v" => (x.wrapping_sub(1), y),
        "^" => (x + 1, y),
        _ => unreachable!(),
    }
}

fn get_n_next_pos(s: &str, (x, y): (usize, usize), n: usize) -> (usize, usize) {
    match s {
        ">" => (x, y + n),
        "<" => (x, y.wrapping_sub(n)),
        "v" => (x + n, y),
        "^" => (x.wrapping_sub(n), y),
        _ => unreachable!(),
    }
}

fn gen_n_positions(
    s: &str,
    (x, y): (usize, usize),
    n: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut res = VecDeque::new();
    for j in 0..n {
        res.push_front(get_n_next_pos(s, (x, y), j));
    }
    res.into_iter()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellA {
    Empty,
    Wall,
    Block,
}
impl Display for CellA {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            CellA::Empty => write!(f, "."),
            CellA::Wall => write!(f, "#"),
            CellA::Block => write!(f, "O"),
        }
    }
}

struct GameA {
    map: Vec<Vec<CellA>>,
    pos: (usize, usize),
}

impl GameA {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            pos: (0, 0),
        }
    }

    fn add_map_line(&mut self, line: &[String]) {
        let mut res = Vec::new();
        for (i, s) in line.iter().enumerate() {
            match s.as_str() {
                "#" => res.push(CellA::Wall),
                "O" => res.push(CellA::Block),
                "." => res.push(CellA::Empty),
                "@" => {
                    res.push(CellA::Empty);
                    self.pos = (self.map.len(), i);
                }
                _ => unreachable!(),
            }
        }

        self.map.push(res);
    }

    fn execute(&mut self, s: &str) {
        let next = get_next_pos(s, self.pos);

        // Early return if move is not safe
        if !self.is_safe(next) {
            return;
        }

        match self.map[next.0][next.1] {
            CellA::Wall => (),
            CellA::Empty => self.pos = next,
            CellA::Block => {
                let (moves, outcome) =
                    std::iter::successors(Some(next), |&curr| Some(get_next_pos(s, curr)))
                        .enumerate()
                        .find_map(|(moves, curr)| {
                            if !self.is_safe(curr) {
                                unreachable!();
                            }
                            match self.map[curr.0][curr.1] {
                                CellA::Wall => Some((moves, false)),
                                CellA::Empty => Some((moves + 1, true)),
                                CellA::Block => None,
                            }
                        })
                        .unwrap_or((0, false));

                if moves > 0 && outcome {
                    for position in gen_n_positions(s, next, moves) {
                        let prev = get_prev_pos(s, position);
                        self.map[position.0][position.1] = self.map[prev.0][prev.1];
                    }

                    self.map[next.0][next.1] = CellA::Empty;
                }

                if outcome {
                    self.pos = next;
                }
            }
        }
    }

    fn is_safe(&self, (x, y): (usize, usize)) -> bool {
        x < self.map.len() && y < self.map[0].len()
    }

    fn calc_score(&self) -> usize {
        let mut sum = 0;
        for i in 1..self.map.len() - 1 {
            for j in 1..self.map[i].len() - 1 {
                if self.map[i][j] == CellA::Block {
                    sum += i * 100 + j;
                }
            }
        }
        sum
    }

    fn print(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Move cursor to top-left and clear screen
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;

        // Print map
        for (i, row) in self.map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if self.pos == (i, j) {
                    print!("@")
                } else {
                    print!("{c}");
                }
            }
            println!()
        }

        stdout.flush()?;
        Ok(())
    }
}

pub fn solve_a() {
    let mut map = true;
    let mut game = GameA::new();
    let visuals = false; // Change this if you want to see the map changing
    read_chars("d15.txt", 24).iter().for_each(|l| {
        if l.is_empty() {
            map = false;
            return;
        }

        if map {
            game.add_map_line(l);
        } else {
            l.iter().for_each(|m| {
                if visuals {
                    let _ = game.print();
                    thread::sleep(Duration::from_millis(50));
                }
                game.execute(m);
            })
        }
    });

    println!("{}", game.calc_score());
}

// Opted to create completely different structs for each
// problem
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellB {
    Empty,
    Wall,
    LBlock,
    RBlock,
}
impl Display for CellB {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            CellB::Empty => write!(f, "."),
            CellB::Wall => write!(f, "#"),
            CellB::LBlock => write!(f, "["),
            CellB::RBlock => write!(f, "]"),
        }
    }
}

struct GameB {
    map: Vec<Vec<CellB>>,
    pos: (usize, usize),
}

impl GameB {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            pos: (0, 0),
        }
    }

    fn add_map_line(&mut self, line: &[String]) {
        let mut res = Vec::new();
        for s in line {
            match s.as_str() {
                "#" => res.extend([CellB::Wall, CellB::Wall]),
                "O" => res.extend([CellB::LBlock, CellB::RBlock]),
                "." => res.extend([CellB::Empty, CellB::Empty]),
                "@" => {
                    res.extend([CellB::Empty, CellB::Empty]);
                    self.pos = (self.map.len(), res.len() - 2);
                }
                _ => unreachable!(),
            }
        }

        self.map.push(res);
    }

    fn execute(&mut self, s: &str) {
        let next = get_next_pos(s, self.pos);
        if !self.is_safe(next) {
            return;
        }

        // Check if free to move
        match self.map[next.0][next.1] {
            CellB::Wall => (),
            CellB::Empty => self.pos = next,
            CellB::RBlock | CellB::LBlock => {
                // If the movement is horizonal, the previous algo applies
                if is_move_horizontal(s) {
                    self.horizontal_execution(s, next);
                } else {
                    self.vertical_execution(s, next);
                }
            }
        }
    }

    // First recursive call, we use same match
    fn vertical_execution(&mut self, s: &str, next: (usize, usize)) {
        let value = self.map[next.0][next.1];
        match value {
            CellB::Wall => (),
            CellB::Empty => self.pos = next,
            CellB::RBlock | CellB::LBlock => {
                // Check for the pair as well
                let pair = match value {
                    CellB::RBlock => (next.0, next.1 - 1),
                    CellB::LBlock => (next.0, next.1 + 1),
                    _ => unreachable!(),
                };

                if self.check_vertical(s, next) && self.check_vertical(s, pair) {
                    self.move_vertical(s, next);
                    self.pos = next;
                }
            }
        }
    }

    fn check_vertical(&mut self, s: &str, curr: (usize, usize)) -> bool {
        let next = get_next_pos(s, curr);
        let value = self.map[next.0][next.1];

        match value {
            CellB::Wall => false,
            CellB::Empty => true,
            CellB::RBlock | CellB::LBlock => {
                // Check for the pair as well
                let pair = match value {
                    CellB::RBlock => (next.0, next.1 - 1),
                    CellB::LBlock => (next.0, next.1 + 1),
                    _ => unreachable!(),
                };

                self.check_vertical(s, next) && self.check_vertical(s, pair)
            }
        }
    }

    fn move_vertical(&mut self, s: &str, next: (usize, usize)) {
        let value = self.map[next.0][next.1];
        match value {
            CellB::Wall => (),
            CellB::Empty => (),
            CellB::RBlock | CellB::LBlock => {
                let pair = match value {
                    CellB::RBlock => (next.0, next.1 - 1),
                    CellB::LBlock => (next.0, next.1 + 1),
                    _ => unreachable!(),
                };
                let nnext = get_next_pos(s, next);
                let npair = get_next_pos(s, pair);

                self.move_vertical(s, nnext);
                self.move_vertical(s, npair);

                self.map[nnext.0][nnext.1] = self.map[next.0][next.1];
                self.map[npair.0][npair.1] = self.map[pair.0][pair.1];

                self.map[next.0][next.1] = CellB::Empty;
                self.map[pair.0][pair.1] = CellB::Empty;
            }
        }
    }

    fn horizontal_execution(&mut self, s: &str, next: (usize, usize)) {
        match self.map[next.0][next.1] {
            CellB::Wall => (),
            CellB::Empty => self.pos = next,
            CellB::RBlock | CellB::LBlock => {
                let (moves, outcome) =
                    std::iter::successors(Some(next), |&curr| Some(get_next_pos(s, curr)))
                        .enumerate()
                        .find_map(|(moves, curr)| {
                            if !self.is_safe(curr) {
                                unreachable!();
                            }
                            match self.map[curr.0][curr.1] {
                                CellB::Wall => Some((moves, false)),
                                CellB::Empty => Some((moves + 1, true)),
                                CellB::RBlock | CellB::LBlock => None,
                            }
                        })
                        .unwrap_or((0, false));

                if moves > 0 && outcome {
                    for position in gen_n_positions(s, next, moves) {
                        let prev = get_prev_pos(s, position);
                        self.map[position.0][position.1] = self.map[prev.0][prev.1];
                    }

                    self.map[next.0][next.1] = CellB::Empty;
                }

                if outcome {
                    self.pos = next;
                }
            }
        }
    }

    fn is_safe(&self, (x, y): (usize, usize)) -> bool {
        x < self.map.len() && y < self.map[0].len()
    }

    fn calc_score(&self) -> usize {
        let mut sum = 0;
        for i in 1..self.map.len() - 1 {
            for j in 1..self.map[i].len() - 1 {
                if self.map[i][j] == CellB::LBlock {
                    sum += i * 100 + j;
                }
            }
        }
        sum
    }

    fn print(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;

        for (i, row) in self.map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if self.pos == (i, j) {
                    print!("@")
                } else {
                    print!("{c}");
                }
            }
            println!()
        }

        stdout.flush()?;
        Ok(())
    }
}

pub fn solve_b() {
    let mut map = true;
    let mut game = GameB::new();
    let visuals = false;
    read_chars("d15.txt", 24).iter().for_each(|l| {
        if l.is_empty() {
            map = false;
            return;
        }

        if map {
            game.add_map_line(l);
        } else {
            l.iter().for_each(|m| {
                if visuals {
                    let _ = game.print();
                    thread::sleep(Duration::from_millis(50));
                }
                game.execute(m);
            });
            if visuals {
                let _ = game.print();
            }
        }
    });

    println!("{}", game.calc_score());
}
