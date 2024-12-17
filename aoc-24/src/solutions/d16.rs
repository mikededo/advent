use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

    fn is_vertical(&self) -> bool {
        *self == Self::Up || *self == Self::Down
    }
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

    // I always have a hard time implementing Djikstra...
    fn find_min_cost(&self, start: Node, end: Node) -> usize {
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

    fn find_all_min(&self, start: Node, end: Node) -> usize {
        let min_cost = self.find_min_cost(start, end);
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        // We will store all the points that are contained in one of the paths that arrive to the
        // end with the minimum cost
        let mut points = HashSet::new();

        dist.insert(start, [min_cost, min_cost]);
        // (cost, (node, direction, visited))
        heap.push(Reverse((0usize, (start, Direction::Right, vec![]))));

        while let Some(Reverse((cost, (node, from, mut visited)))) = heap.pop() {
            let axis = from.is_vertical() as usize;
            visited.push(node);

            // The cost is already higher than the minimum found
            if cost > dist[&node][axis] || cost > min_cost {
                continue;
            }

            if node == end {
                if cost == min_cost {
                    points.extend(visited);
                }
                continue;
            }

            for to in Direction::ALL {
                let next = next_direction(node, to);
                let cost = cost + if from == to { 1 } else { 1001 };

                if self.map[next.0][next.1] == '#' {
                    continue;
                }

                let entry = dist.entry(next).or_insert([min_cost, min_cost]);
                if cost <= entry[axis] {
                    dist.entry(next).and_modify(|e| e[axis] = cost);
                    heap.push(Reverse((cost, (next, to, visited.clone()))));
                }
            }
        }

        points.len()
    }
}

fn init() -> (Map, (usize, usize), (usize, usize)) {
    let mut g = Map::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = read_lines("16.txt", 24);
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

    (g, start, end)
}

pub fn solve_a() {
    let (g, start, end) = init();
    println!("{:?}", g.find_min_cost(start, end));
}

pub fn solve_b() {
    let (g, start, end) = init();
    println!("{:?}", g.find_all_min(start, end));
}
