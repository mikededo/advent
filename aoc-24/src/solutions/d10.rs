use utils::read_bytes;

struct Trail {
    map: Vec<Vec<u8>>,
    starting_positions: Vec<(usize, usize)>,
    track_visited: bool,
}

impl Trail {
    fn new(track_visited: bool) -> Self {
        Self {
            map: Vec::new(),
            starting_positions: Vec::new(),
            track_visited,
        }
    }

    fn add_line(&mut self, row: usize, line: &[u8]) {
        line.iter().enumerate().for_each(|(col, height)| {
            if col == 0 {
                self.map.push(Vec::new());
            }

            let n = height - b'0';
            if n == 0 {
                self.starting_positions.push((row, col));
            }
            self.map[row].push(n);
        });
    }

    fn count_trails(&self) -> usize {
        self.starting_positions.iter().fold(0, |acc, position| {
            let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
            acc + self.find_trail(&mut visited, *position, 0)
        })
    }

    fn find_trail(
        &self,
        visited: &mut Vec<Vec<bool>>,
        (row, col): (usize, usize),
        depth: usize,
    ) -> usize {
        if visited[row][col] && self.track_visited {
            return 0;
        }

        visited[row][col] = true;
        if depth == 9 {
            return 1;
        }

        // Top
        let mut count = 0;
        if row > 0 {
            let top = self.map[row - 1][col] as usize;
            if top == depth + 1 {
                count += self.find_trail(visited, (row - 1, col), depth + 1);
            }
        }
        // Bottom
        if let Some(line) = self.map.get(row + 1) {
            let bottom = line[col] as usize;
            if bottom == depth + 1 {
                count += self.find_trail(visited, (row + 1, col), depth + 1);
            }
        }
        // Left
        if col > 0 {
            let left = self.map[row][col - 1] as usize;
            if left == depth + 1 {
                count += self.find_trail(visited, (row, col - 1), depth + 1);
            }
        }
        // Right
        if let Some(value) = self.map[row].get(col + 1) {
            let right = *value as usize;
            if right == depth + 1 {
                count += self.find_trail(visited, (row, col + 1), depth + 1);
            }
        }

        count
    }
}

pub fn solve_a() {
    let mut trail = Trail::new(true);
    read_bytes("d10.txt", 24)
        .iter()
        .enumerate()
        .for_each(|(row, line)| trail.add_line(row, line));

    println!("{}", trail.count_trails());
}

pub fn solve_b() {
    let mut trail = Trail::new(false);
    read_bytes("d10.txt", 24)
        .iter()
        .enumerate()
        .for_each(|(row, line)| trail.add_line(row, line));

    println!("{}", trail.count_trails());
}
