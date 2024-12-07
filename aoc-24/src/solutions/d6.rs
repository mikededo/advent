use std::collections::HashSet;

use utils::read_chars_with;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone)]
struct Movement {
    direction: Direction,
}

impl Movement {
    fn new() -> Self {
        Self {
            direction: Direction::Up,
        }
    }

    fn rotate_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn next_pos(&mut self, (x, y): (usize, usize)) -> (usize, usize) {
        match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

fn calc_path(map: &[Vec<String>], start_point: &(usize, usize)) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut unique_count = 0;

    let row_edge = map[0].len() - 1;
    let col_edge = map.len() - 1;
    let is_edge = |i: usize, j: usize| i == 0 || i == row_edge || j == 0 || j == col_edge;

    let (mut row, mut col) = start_point;
    let mut movement = Movement::new();
    while !is_edge(row, col) {
        // We first move to the next position
        let prev_pos = (row, col);
        unique_count += (!visited[row][col]) as usize;
        visited[row][col] = true;
        (col, row) = movement.next_pos((col, row));

        // We now check if it's a valid movement
        if &map[row][col] == "#" {
            // Rotate right 90 degres
            movement.rotate_right();
            (row, col) = prev_pos;
        }
    }

    // Add the final position
    unique_count + 1
}

pub fn solve_a() {
    let mut start_point: Option<(usize, usize)> = None;
    let map = read_chars_with("d6.txt", 24, |c, (row, col)| {
        if c == '^' {
            start_point = Some((row, col));
        }
    });
    let res = match start_point {
        None => panic!("No start point found"),
        Some(point) => calc_path(&map, &point),
    };

    println!("{res}");
}

fn is_loop(map: &[Vec<String>], start_point: &(usize, usize), obstruction: (usize, usize)) -> bool {
    let mut visited_positions = HashSet::new();
    let mut current_map = map.to_vec();
    current_map[obstruction.0][obstruction.1] = "#".to_string();

    let row_edge = map[0].len() - 1;
    let col_edge = map.len() - 1;
    let is_edge = |i: usize, j: usize| i == 0 || i == row_edge || j == 0 || j == col_edge;

    let (mut row, mut col) = start_point;
    let mut movement = Movement::new();

    while !is_edge(row, col) {
        // Check if current path is repeating
        let current_state = (row, col, movement.direction);
        if !visited_positions.insert(current_state) {
            return true;
        }

        let prev_pos = (row, col);
        (col, row) = movement.next_pos((col, row));

        // Check for wall collision
        if current_map[row][col] == "#" {
            movement.rotate_right();
            (row, col) = prev_pos;
        }
    }

    false
}

pub fn solve_b() {
    let mut start_point: Option<(usize, usize)> = None;
    let map = read_chars_with("d6.txt", 24, |c, (row, col)| {
        if c == '^' {
            start_point = Some((row, col));
        }
    });
    let start_point = match start_point {
        None => panic!("No start point found"),
        Some(point) => point,
    };

    // Find all possible obstructions
    let res = map.iter().enumerate().fold(0, |acc, (row_i, row)| {
        let mut count = 0;
        row.iter().enumerate().for_each(|(col_i, col)| {
            // Skip the starting point
            if (row_i, col_i) == start_point {
                return;
            }

            // Skip existing walls
            if col == "#" {
                return;
            }

            // Check if this position causes a loop
            if is_loop(&map, &start_point, (row_i, col_i)) {
                count += 1;
            }
        });
        acc + count
    });

    println!("{res}");
}
