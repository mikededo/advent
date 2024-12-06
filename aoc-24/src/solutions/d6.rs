use utils::read_chars_with;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
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
