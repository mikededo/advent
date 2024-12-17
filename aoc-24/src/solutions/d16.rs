use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use utils::read_lines;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];
}

fn next_direction((x, y): (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x.wrapping_sub(1), y),
        Direction::Down => (x + (1), y),
        Direction::Left => (x, y.wrapping_sub(1)),
        Direction::Right => (x, y + 1),
    }
}

fn next_directions(point: (usize, usize)) -> [(usize, usize); 4] {
    [
        next_direction(point, Direction::Up),
        next_direction(point, Direction::Down),
        next_direction(point, Direction::Left),
        next_direction(point, Direction::Right),
    ]
}

type Node = (usize, usize);

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
}
impl Map {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn map_char(&mut self, row: usize, c: char) {
        if row >= self.map.len() {
            self.map.push(Vec::new());
        }

        self.map[row].push(c);
    }

    // I always have a hard time implementing Djiktra...
    fn find_path(&self, start: Node, end: Node) -> usize {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut min_cost = usize::MAX;

        // Add the starting node with no distance
        dist.insert(start, 0);
        heap.push(Reverse((0usize, (start, Direction::Right))));

        while let Some(Reverse((cost, (node, from)))) = heap.pop() {
            // Are we in destination? And is the cost smaller?
            if node == end && cost < min_cost {
                min_cost = cost;
                continue;
            }

            // We did not make it, yet, but the cost is already higher than the minimum found
            if cost > dist[&node] || cost >= min_cost {
                continue;
            }

            // For all destinations
            for to in Direction::ALL {
                let next = next_direction(node, to);
                let cost = cost + if from == to { 1 } else { 1001 };

                if self.map[next.0][next.1] == '#' {
                    continue;
                }

                if cost < *dist.get(&next).unwrap_or(&usize::MAX) {
                    dist.insert(next, cost);
                    heap.push(Reverse((cost, (next, to))));
                }
            }
        }

        min_cost
    }
}

pub fn solve_a() {
    let mut g = Map::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = read_lines("d16.txt", 24);
    map.iter().enumerate().for_each(|(i, row)| {
        row.chars().enumerate().for_each(|(j, c)| {
            match c {
                'S' => start = (i, j),
                'E' => end = (i, j),
                _ => (),
            }

            g.map_char(i, c);
        })
    });

    println!("{:?}", g.find_path(start, end));
}
