use std::collections::{HashSet, VecDeque};

use utils::read_lines;

fn adjacent((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [
        (x.wrapping_sub(1), y),
        (x + (1), y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            map: vec![vec!['.'; cols]; rows],
        }
    }

    fn find_shortest(&self, end: (usize, usize)) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(((0, 0), 0));
        visited.insert((0, 0));

        while let Some((curr, steps)) = queue.pop_front() {
            if curr == end {
                return steps;
            }

            for next in adjacent(curr) {
                if !self.is_safe(next) || visited.contains(&next) {
                    continue;
                }

                visited.insert(next);
                queue.push_back(((next), steps + 1));
            }
        }

        unreachable!("BFS did not find a path!");
    }

    fn is_safe(&self, (x, y): (usize, usize)) -> bool {
        x < self.map.len() && y < self.map[0].len() && self.map[x][y] != '#'
    }
}

pub fn solve_a() {
    let size = 71;
    let mut map = Map::new(size, size);

    read_lines("d18.txt", 24)[..1024].iter().for_each(|line| {
        let split = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        map.map[split[1]][split[0]] = '#';
    });

    println!("{}", map.find_shortest((size - 1, size - 1)));
}
