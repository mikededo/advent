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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Block,
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Block => write!(f, "O"),
        }
    }
}

struct Game {
    map: Vec<Vec<Cell>>,
    pos: (usize, usize),
}

impl Game {
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
                "#" => res.push(Cell::Wall),
                "O" => res.push(Cell::Block),
                "." => res.push(Cell::Empty),
                "@" => {
                    res.push(Cell::Empty);
                    self.pos = (self.map.len(), i);
                }
                _ => unreachable!(),
            }
        }

        self.map.push(res);
    }

    fn execute(&mut self, s: &str) {
        let next = Game::get_next_pos(s, self.pos);
        if !self.is_safe(next) {
            return;
        }

        // Check if free to move
        match self.map[next.0][next.1] {
            Cell::Wall => (),
            Cell::Empty => self.pos = next,
            Cell::Block => {
                // In the same direction, find if there's one free space after 1+ blocks
                let mut is_empty = false;
                let mut is_blocked = false;
                let mut moves = 0;
                let mut curr = next;
                while !is_empty && !is_blocked {
                    moves += 1;

                    if !self.is_safe(curr) {
                        unreachable!();
                    }

                    match self.map[curr.0][curr.1] {
                        Cell::Wall => is_blocked = true,
                        Cell::Empty => is_empty = true,
                        Cell::Block => (),
                    }
                    curr = Game::get_next_pos(s, curr);
                }

                if moves > 0 && !is_blocked {
                    for position in Game::gen_n_positions(s, next, moves) {
                        let prev = Game::get_prev_pos(s, position);
                        self.map[position.0][position.1] = self.map[prev.0][prev.1];
                    }
                    // Curr cell as empty
                    self.map[next.0][next.1] = Cell::Empty;
                }

                if !is_blocked {
                    self.pos = next
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
                if self.map[i][j] == Cell::Block {
                    sum += i * 100 + j;
                }
            }
        }
        sum
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
            res.push_front(Game::get_n_next_pos(s, (x, y), j));
        }
        res.into_iter()
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
    let mut game = Game::new();
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
                game.execute(m);
                if visuals {
                    let _ = game.print();
                    thread::sleep(Duration::from_millis(50));
                }
            })
        }
    });

    println!("{}", game.calc_score());
}
